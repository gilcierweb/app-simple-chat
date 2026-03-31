import { defineStore } from 'pinia'
import type { User, Profile } from '~/types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const profile = ref<Profile | null>(null)
  const isHydrated = ref(false)

  const isAuthenticated = computed(() => !!user.value)

  function setUser(u: User | null) {
    user.value = u
  }

  function setProfile(p: Profile | null) {
    profile.value = p
  }

  function hydrate() {
    if (!import.meta.client) return
    // Restore user from localStorage token if available
    const token = localStorage.getItem('access_token')
    if (!token) { isHydrated.value = true; return }

    try {
      // Decode JWT payload (no verification — just read claims)
      const payload = JSON.parse(atob(token.split('.')[1]))
      if (payload.exp * 1000 < Date.now()) {
        localStorage.removeItem('access_token')
        localStorage.removeItem('refresh_token')
      }
    } catch {}
    isHydrated.value = true
  }

  function logout() {
    user.value = null
    profile.value = null
    if (import.meta.client) {
      localStorage.removeItem('access_token')
      localStorage.removeItem('refresh_token')
    }
  }

  return { user, profile, isAuthenticated, isHydrated, setUser, setProfile, hydrate, logout }
})
