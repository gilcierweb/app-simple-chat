import { defineEventHandler, getCookie, setCookie, createError } from 'h3'

export default defineEventHandler(async (event) => {
  const refreshToken = getCookie(event, 'auth_refresh')

  if (!refreshToken) {
    throw createError({ statusCode: 401, message: 'No refresh token available' })
  }

  // Fetch Rust backend
  const config = useRuntimeConfig()
  const apiBase = config.public?.apiBase || 'http://localhost:8080/api/v1'

  try {
    const data: any = await $fetch(`${apiBase}/auth/refresh`, {
      method: 'POST',
      body: { refresh_token: refreshToken }
    })

    if (data.refresh_token) {
      setCookie(event, 'auth_refresh', data.refresh_token, {
        httpOnly: true,
        secure: process.env.NODE_ENV === 'production',
        sameSite: 'lax',
        maxAge: 60 * 60 * 24 * 30, // 30 days
        path: '/'
      })
    }

    return {
      access_token: data.access_token
    }
  } catch (error: any) {
    // If refresh fails (e.g., token expired/invalid), clear the cookie
    setCookie(event, 'auth_refresh', '', {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      maxAge: -1,
      path: '/'
    })
    
    throw createError({
      statusCode: 401,
      message: 'Failed to refresh token'
    })
  }
})
