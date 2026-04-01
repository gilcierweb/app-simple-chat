import type { AuthResponse, User } from '~/types'
import { useAuthStore } from '~/stores/auth'

/**
 * Authentication composable — handles login, register, token management.
 * Tokens are stored in localStorage; auth state is derived from them.
 */
export const useAuth = () => {
  const config = useRuntimeConfig()
  const router = useRouter()
  const authStore = useAuthStore()

  const user = useState<User | null>('auth:user', () => null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const accessToken = computed(() => {
    if (import.meta.client) return localStorage.getItem('access_token')
    return null
  })

  const isAuthenticated = computed(() => !!user.value && !!accessToken.value)

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
      error.value = e?.data?.message || 'Registration failed'
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

      localStorage.setItem('access_token', data.access_token)
      localStorage.setItem('refresh_token', data.refresh_token)
      user.value = data.user
      authStore.setUser(data.user)

      // Upload keys if not yet done
      const keyStore = useKeyStore()
      await keyStore.ensureKeys(data.access_token)

      await router.push('/chat')
    } catch (e: any) {
      error.value = e?.data?.message || 'Login failed'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    const token = localStorage.getItem('access_token')
    if (token) {
      try {
        await $fetch(`${config.public.apiBase}/auth/logout`, {
          method: 'POST',
          headers: { Authorization: `Bearer ${token}` },
        })
      } catch {}
    }
    localStorage.removeItem('access_token')
    localStorage.removeItem('refresh_token')
    user.value = null
    authStore.setUser(null)
    await router.push('/auth/login')
  }

  async function refreshAccessToken(): Promise<string | null> {
    const refreshToken = localStorage.getItem('refresh_token')
    if (!refreshToken) return null
    try {
      const data = await $fetch<{ access_token: string; refresh_token: string }>(
        `${config.public.apiBase}/auth/refresh`,
        { method: 'POST', body: { refresh_token: refreshToken } },
      )
      localStorage.setItem('access_token', data.access_token)
      localStorage.setItem('refresh_token', data.refresh_token)
      return data.access_token
    } catch {
      await logout()
      return null
    }
  }

  /**
   * Perform an authenticated API fetch with automatic token refresh.
   */
  async function authFetch<T>(url: string, opts: RequestInit = {}): Promise<T> {
    let token = localStorage.getItem('access_token')
    
    const makeRequest = async (authToken: string): Promise<T> => {
      return await $fetch<T>(`${config.public.apiBase}${url}`, {
        ...opts,
        headers: {
          'Content-Type': 'application/json',
          ...opts.headers,
          Authorization: `Bearer ${authToken}`,
        },
      })
    }

    try {
      return await makeRequest(token!)
    } catch (e: any) {
      if (e.response?.status === 401) {
        token = await refreshAccessToken()
        if (!token) throw new Error('Unauthenticated')
        return await makeRequest(token)
      }
      throw e
    }
  }

  return { user, loading, error, isAuthenticated, register, login, logout, authFetch }
}
