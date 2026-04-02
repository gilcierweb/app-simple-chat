import type { AuthRouteMeta } from '~/types'
import { useAuthStore } from '~/stores/auth'

declare module 'vue-router' {
  interface RouteMeta extends AuthRouteMeta {}
}

/**
 * Global Authentication Middleware
 * 
 * SSR-friendly route protection. 
 * pinia-plugin-persistedstate/nuxt automatically hydrates the store from cookies 
 * on both client and server before route middleware runs.
 */

// Routes that are always public (no auth check needed)
const ALWAYS_PUBLIC_PATHS = ['/', '/404', '/_error']

export default defineNuxtRouteMiddleware((to, from) => {
  const path = to.path
  
  // Always allow public paths
  if (ALWAYS_PUBLIC_PATHS.includes(path)) {
    return
  }
  
  // Get route meta
  const meta = (to.meta || {}) as AuthRouteMeta
  const requiresAuth = meta.requiresAuth ?? false
  const guestOnly = meta.guestOnly ?? path.startsWith('/auth/')
  const loginRedirect = meta.loginRedirect || '/auth/login'
  const homeRedirect = meta.homeRedirect || '/chat'
  const requiredRoles = meta.requiredRoles || []
  
  // Read auth directly from the Pinia store.
  const authStore = useAuthStore()
  const isAuthenticated = authStore.isAuthenticated && !authStore.isTokenExpired
  const userRoles = authStore.userRoles
  
  // Case 1: Route requires authentication
  if (requiresAuth) {
    if (!isAuthenticated) {
      const redirectPath = encodeURIComponent(to.fullPath)
      return navigateTo(`${loginRedirect}?redirect=${redirectPath}`)
    }
    
    // Check role requirements
    if (requiredRoles.length > 0) {
      const hasRequiredRole = requiredRoles.some(role => 
        userRoles.includes(role) || userRoles.includes('admin')
      )
      if (!hasRequiredRole) {
        return navigateTo(homeRedirect)
      }
    }
    
    return
  }
  
  // Case 2: Guest-only route (login pages)
  if (guestOnly && isAuthenticated) {
    return navigateTo(homeRedirect)
  }
  
  // Case 3: Public route - allow access
})
