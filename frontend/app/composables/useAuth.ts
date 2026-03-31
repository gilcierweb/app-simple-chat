import type { AuthResponse, User } from '~/types'

/**
 * Authentication composable — handles login, register, token management.
 * Tokens are stored in localStorage; auth state is derived from them.
 */
export const useAuth = () => {
  const config = useRuntimeConfig()
  const router = useRouter()

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
      const res = await $fetch(`${config.public.apiBaseUrl}/auth/register`, {
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
      const data = await $fetch<AuthResponse>(`${config.public.apiBaseUrl}/auth/login`, {
        method: 'POST',
        body: { email, password, totp_code: totpCode || undefined },
      })

      localStorage.setItem('access_token', data.access_token)
      localStorage.setItem('refresh_token', data.refresh_token)
      user.value = data.user

      // Upload keys if not yet done
      const keyStore = useKeyStore()
      await keyStore.ensureKeys(data.access_token)

      await router.push('/')
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
        await $fetch(`${config.public.apiBaseUrl}/auth/logout`, {
          method: 'POST',
          headers: { Authorization: `Bearer ${token}` },
        })
      } catch {}
    }
    localStorage.removeItem('access_token')
    localStorage.removeItem('refresh_token')
    user.value = null
    await router.push('/auth/login')
  }

  async function refreshAccessToken(): Promise<string | null> {
    const refreshToken = localStorage.getItem('refresh_token')
    if (!refreshToken) return null
    try {
      const data = await $fetch<{ access_token: string; refresh_token: string }>(
        `${config.public.apiBaseUrl}/auth/refresh`,
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
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(opts.headers as Record<string, string>),
      Authorization: `Bearer ${token}`,
    }

    let res = await fetch(`${config.public.apiBaseUrl}${url}`, { ...opts, headers })

    if (res.status === 401) {
      token = await refreshAccessToken()
      if (!token) throw new Error('Unauthenticated')
      headers.Authorization = `Bearer ${token}`
      res = await fetch(`${config.public.apiBaseUrl}${url}`, { ...opts, headers })
    }

    if (!res.ok) {
      const body = await res.json().catch(() => ({}))
      throw new Error(body.message || `HTTP ${res.status}`)
    }
    return res.json() as T
  }

  return { user, loading, error, isAuthenticated, register, login, logout, authFetch }
}
