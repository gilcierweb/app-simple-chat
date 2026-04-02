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

export default defineNuxtRouteMiddleware(async (to, from) => {
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

  // Session Hydration: If accessToken is missing or expired, but we have a user (from persist)
  if ((!authStore.accessToken || authStore.isTokenExpired) && authStore.user && path !== '/auth/login') {
    // 1. SILENT REFRESH (Only on Client)
    // The Nuxt server (SSR) doesn't have access to browser-held HttpOnly cookies from :8080.
    // So we skip the refresh call on server-side to avoid unnecessary failure and logout.
    if (import.meta.client) {
      try {
        const config = useRuntimeConfig()
        const fetcher = $fetch
        
        console.log('[AuthMiddleware] Attempting silent refresh via HttpOnly cookie...')
        const data = await fetcher<{ access_token: string, refresh_token: string }>(
          `${config.public.apiBase}/auth/refresh`, 
          { 
            method: 'POST',
            body: authStore.refreshToken ? { refresh_token: authStore.refreshToken } : {},
            credentials: 'include', // Crucial for HttpOnly cookies
          }
        )
        
        authStore.setTokens(data.access_token, data.refresh_token)
      } catch (e) {
        console.error('[AuthMiddleware] Silent refresh failed on client:', e)
        // Only logout if on client and the refresh actually failed (invalid session)
        if (requiresAuth) {
          authStore.logout()
        }
      } finally {
        // Hydration phase is over, back to strict token checks
        authStore.isInitialHydration = false
      }
    } else {
      // On Server (SSR): We just stay quiet. 
      // We don't have the cookie, but we have the user info from 'pinia_auth' cookie (Lax/not HttpOnly).
      // We allow the render to proceed, trusting the client-side middleware to handle the refresh.
      console.log('[AuthMiddleware] SSR detected: deferring refresh to client-side...')
    }
  }

  const isAuthenticated = authStore.isAuthenticated && !authStore.isTokenExpired
  // On the server, if we have a user object, we treat them as "authenticated for SSR purposes"
  // This prevents the server from redirecting to login before the client-side silent refresh
  // has a chance to run and obtain a fresh accessToken via HttpOnly cookies.
  const isServerWithUser = import.meta.server && !!authStore.user
  const userRoles = authStore.userRoles
  
  // Case 1: Route requires authentication
  if (requiresAuth) {
    if (!isAuthenticated && !isServerWithUser) {
      const redirectPath = encodeURIComponent(to.fullPath)
      return navigateTo(`${loginRedirect}?redirect=${redirectPath}`)
    }
    
    // Check role requirements (only if truly authenticated or on client)
    // On server, we skip role checks if we're in this "defer" state
    if (isAuthenticated && requiredRoles.length > 0) {
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
