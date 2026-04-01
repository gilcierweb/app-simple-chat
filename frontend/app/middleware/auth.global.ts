export default defineNuxtRouteMiddleware((to) => {
  if (!import.meta.client) return

  const token = localStorage.getItem('access_token')
  const publicRoutes = ['/','/auth/login', '/auth/register', '/auth/confirm', '/auth/forgot-password', '/auth/reset-password']

  if (!token && !publicRoutes.some(r => to.path.startsWith(r))) {
    return navigateTo('/auth/login')
  }

  if (token && to.path.startsWith('/auth/')) {
    return navigateTo('/chat')
  }
})
