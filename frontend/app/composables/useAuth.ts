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
    let loginSuccess = false
    let targetPath = '/chat'
    
    try {
      const data = await $fetch<AuthResponse>(`${config.public.apiBase}/auth/login`, {
        method: 'POST',
        body: { email, password, totp_code: totpCode || undefined },
        credentials: 'include', // Crucial for HttpOnly cookies
      })

      // Store tokens in Pinia (access_token stays in memory)
      authStore.setTokens(data.access_token, data.refresh_token)
      authStore.setUser(data.user)

      // Upload keys if not yet done
      const keyStore = useKeyStore()
      await keyStore.ensureKeys(data.access_token)

      // Determine redirect target
      const redirect = route.query.redirect as string
      targetPath = redirect ? decodeURIComponent(redirect) : '/chat'
      loginSuccess = true
      
    } catch (e: any) {
      error.value = e?.data?.message || t('auth.errors.loginFailed')
      throw e
    } finally {
      loading.value = false
    }
    
    // Navigate after successful login (outside try-catch)
    if (loginSuccess) {
      return navigateTo(targetPath)
    }
  }

  async function logout(preventRedirect = false) {
    const token = authStore.accessToken
    if (token) {
      try {
        await $fetch(`${config.public.apiBase}/auth/logout`, {
          method: 'POST',
          headers: { Authorization: `Bearer ${token}` },
          credentials: 'include', // Clear the backend HttpOnly cookie
        })
      } catch {}
    }
    
    // Clear everything via store (cookies cleared automatically)
    authStore.logout()
    
    if (!preventRedirect) {
      if (import.meta.client) {
        await router.push('/auth/login')
      } else {
        return navigateTo('/auth/login')
      }
    }
  }

  async function refreshAccessToken(): Promise<string | null> {
    try {
      const fetcher = import.meta.client ? $fetch : useRequestFetch()
      const data = await fetcher<{ access_token: string }>(`${config.public.apiBase}/auth/refresh`, { 
        method: 'POST',
        credentials: 'include',
      })
      
      // Update tokens in store 
      authStore.setTokens(data.access_token, null)
      
      return data.access_token
    } catch {
      await logout(true)
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

  return { user, loading, error, isAuthenticated, accessToken, register, login, logout, authFetch, refreshAccessToken }
}
