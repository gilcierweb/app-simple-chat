import type { AuthRouteMeta } from '~/types'

declare module 'vue-router' {
  interface RouteMeta extends AuthRouteMeta {}
}

/**
 * Global Authentication Middleware
 * 
 * SSR-friendly route protection using cookies via pinia-plugin-persistedstate/nuxt.
 * 
 * IMPORTANT: This middleware runs on BOTH server and client.
 * - On server: reads cookies directly via useCookie
 * - On client: uses Pinia store (already hydrated from cookies)
 */

// Routes that are always public (no auth check needed)
const ALWAYS_PUBLIC_PATHS = ['/', '/404', '/_error']

interface JwtPayload {
  sub: string
  exp: number
  iat: number
  roles?: string[]
}

function parseJwt(token: string): JwtPayload | null {
  try {
    const parts = token.split('.')
    if (parts.length !== 3) return null
    
    const payloadPart = parts[1]
    if (!payloadPart) return null
    
    const padding = 4 - (payloadPart.length % 4)
    const base64 = padding === 4 ? payloadPart : payloadPart + '='.repeat(padding)
    
    return JSON.parse(atob(base64.replace(/-/g, '+').replace(/_/g, '/')))
  } catch {
    return null
  }
}

function isTokenValid(token: string | null | undefined): boolean {
  if (!token) return false
  const payload = parseJwt(token)
  if (!payload) return false
  return payload.exp * 1000 > Date.now()
}

export default defineNuxtRouteMiddleware(async (to, from) => {
  const path = to.path
  
  // Check if this is an always public path (exact match only)
  const isAlwaysPublic = ALWAYS_PUBLIC_PATHS.includes(path)
  if (isAlwaysPublic) {
    return
  }
  
  // Get route meta with defaults
  const meta = (to.meta || {}) as AuthRouteMeta
  
  // Determine route requirements
  const requiresAuth = meta.requiresAuth ?? false
  const guestOnly = meta.guestOnly ?? path.startsWith('/auth/')
  const loginRedirect = meta.loginRedirect || '/auth/login'
  const homeRedirect = meta.homeRedirect || '/chat'
  const requiredRoles = meta.requiredRoles || []
  
  // Check authentication based on environment
  let isAuthenticated = false
  let userRoles: string[] = []
  
  if (import.meta.server) {
    // SERVER-SIDE: Read cookies directly
    const accessToken = useCookie<string | undefined>('auth.accessToken').value
    
    isAuthenticated = isTokenValid(accessToken)
    
    if (isAuthenticated && accessToken) {
      const payload = parseJwt(accessToken)
      userRoles = payload?.roles || []
    }
  } else {
    // CLIENT-SIDE: Use Pinia store (already hydrated from cookies)
    const authStore = useAuthStore()
    isAuthenticated = authStore.isAuthenticated && !authStore.isTokenExpired
    userRoles = authStore.userRoles
  }
  
  // Case 1: Route requires authentication
  if (requiresAuth) {
    if (!isAuthenticated) {
      // Save intended destination for post-login redirect
      const redirectPath = encodeURIComponent(to.fullPath)
      return navigateTo(`${loginRedirect}?redirect=${redirectPath}`, { replace: true })
    }
    
    // Check role requirements
    if (requiredRoles.length > 0) {
      const hasRequiredRole = requiredRoles.some(role => 
        userRoles.includes(role) || userRoles.includes('admin')
      )
      if (!hasRequiredRole) {
        return navigateTo(homeRedirect, { replace: true })
      }
    }
    
    // User is authenticated and authorized - allow access
    return
  }
  
  // Case 2: Route is guest-only (e.g., login/register pages)
  if (guestOnly && isAuthenticated) {
    // Authenticated user trying to access login page - redirect to home
    return navigateTo(homeRedirect, { replace: true })
  }
  
  // Case 3: Public route with no special requirements - allow access
})
