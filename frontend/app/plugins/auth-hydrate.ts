import type { User } from '~/types'

export default defineNuxtPlugin({
  name: 'auth-hydrate',
  setup() {
    if (!import.meta.client) return

    const authStore = useAuthStore()
    const token = localStorage.getItem('access_token')
    
    if (!token) {
      authStore.hydrate()
      return
    }

    try {
      const parts = token.split('.')
      if (parts.length !== 3) throw new Error('Invalid token')
      
      const payloadPart = parts[1]
      if (!payloadPart) throw new Error('Invalid token')
      
      const payload = JSON.parse(atob(payloadPart))
      
      if (payload.exp * 1000 < Date.now()) {
        localStorage.removeItem('access_token')
        localStorage.removeItem('refresh_token')
      } else if (payload.sub) {
        const user: User = {
          id: payload.sub,
          email: '',
          confirmed_at: null,
          totp_enabled: false,
          created_at: '',
        }
        authStore.setUser(user)
      }
    } catch {
      // Invalid token
    } finally {
      authStore.hydrate()
    }
  },
})
