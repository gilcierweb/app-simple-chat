import type { AuthResponse } from '~/types'
import { useAuthStore } from '~/stores/auth'

/**
 * Authentication composable — handles login, register, token management.
 * Uses Pinia store with cookie persistence (SSR-friendly via pinia-plugin-persistedstate/nuxt)
 */
export const useAuth = () => {
  const config = useRuntimeConfig()
  const router = useRouter()
  const authStore = useAuthStore()
  const route = useRoute()

  const user = computed(() => authStore.user)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const { t } = useI18n()

  const accessToken = computed(() => authStore.accessToken)

  const isAuthenticated = computed(() => authStore.isAuthenticated)

  async function register(email: string, password: string) {
    loading.value = true
    error.value = null
    try {
      const res = await $fetch(`${config.public.apiBase}/auth/register`, {
        method: 'POST',
        body: { email, password },
      })
      return res
    } catch (e: any) {
      error.value = e?.data?.message || t('auth.errors.registrationFailed')
      throw e
    } finally {
      loading.value = false
    }
  }

  async function login(email: string, password: string, totpCode?: string) {
    loading.value = true
    error.value = null
    try {
      const data = await $fetch<AuthResponse>(`${config.public.apiBase}/auth/login`, {
        method: 'POST',
        body: { email, password, totp_code: totpCode || undefined },
      })

      // Store tokens in Pinia (automatically persisted to cookies)
      authStore.setTokens(data.access_token, data.refresh_token)
      authStore.setUser(data.user)

      // Upload keys if not yet done
      const keyStore = useKeyStore()
      await keyStore.ensureKeys(data.access_token)

      // Handle redirect after login
      const redirect = route.query.redirect as string
      if (redirect) {
        await router.push(decodeURIComponent(redirect))
      } else {
        await router.push('/chat')
      }
    } catch (e: any) {
      error.value = e?.data?.message || t('auth.errors.loginFailed')
      throw e
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    const token = authStore.accessToken
    if (token) {
      try {
        await $fetch(`${config.public.apiBase}/auth/logout`, {
          method: 'POST',
          headers: { Authorization: `Bearer ${token}` },
        })
      } catch {}
    }
    
    // Clear everything via store (cookies cleared automatically)
    authStore.logout()
    await router.push('/auth/login')
  }

  async function refreshAccessToken(): Promise<string | null> {
    const refreshToken = authStore.refreshToken
    if (!refreshToken) return null
    try {
      const data = await $fetch<{ access_token: string; refresh_token: string }>(
        `${config.public.apiBase}/auth/refresh`,
        { method: 'POST', body: { refresh_token: refreshToken } },
      )
      
      // Update tokens in store (automatically persisted to cookies)
      authStore.setTokens(data.access_token, data.refresh_token)
      
      return data.access_token
    } catch {
      await logout()
      return null
    }
  }

  /**
   * Perform an authenticated API fetch with automatic token refresh.
   */
  async function authFetch<T>(url: string, opts: { method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'; body?: any; headers?: Record<string, string> } = {}): Promise<T> {
    let token = authStore.accessToken
    
    const makeRequest = async (authToken: string): Promise<T> => {
      return await $fetch<T>(`${config.public.apiBase}${url}`, {
        method: opts.method || 'GET',
        headers: {
          'Content-Type': 'application/json',
          ...opts.headers,
          Authorization: `Bearer ${authToken}`,
        },
        body: opts.body,
      })
    }

    try {
      return await makeRequest(token!)
    } catch (e: any) {
      if (e.response?.status === 401) {
        token = await refreshAccessToken()
        if (!token) throw new Error(t('auth.errors.unauthenticated'))
        return await makeRequest(token)
      }
      throw e
    }
  }

  return { user, loading, error, isAuthenticated, accessToken, register, login, logout, authFetch }
}
