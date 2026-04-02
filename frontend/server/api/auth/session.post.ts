import { defineEventHandler, readBody, setCookie } from 'h3'

export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  const refreshToken = body.refresh_token

  if (!refreshToken) {
    return { success: false, message: 'Refresh token is required' }
  }

  // Set the refresh token in an HttpOnly, Secure cookie
  setCookie(event, 'auth_refresh', refreshToken, {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'lax',
    maxAge: 60 * 60 * 24 * 30, // 30 days
    path: '/'
  })

  return { success: true }
})
