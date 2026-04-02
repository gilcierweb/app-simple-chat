import { defineEventHandler, getCookie, setCookie } from 'h3'

export default defineEventHandler(async (event) => {
  const authHeader = event.node.req.headers.authorization

  const config = useRuntimeConfig()
  const apiBase = config.public?.apiBase || 'http://localhost:8080/api/v1'

  // Attempt to call Rust logout
  if (authHeader) {
    try {
      await $fetch(`${apiBase}/auth/logout`, {
        method: 'POST',
        headers: { Authorization: authHeader }
      })
    } catch (e) {
      console.error('Rust backend logout failed:', e)
    }
  }

  // Destroy the local HttpOnly cookie
  setCookie(event, 'auth_refresh', '', {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'lax',
    maxAge: -1,
    path: '/'
  })

  return { success: true }
})
