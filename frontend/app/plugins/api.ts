export default defineNuxtPlugin((nuxtApp) => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiBase || 'http://localhost:8080/api/v1'

  const authApi = async <T>(url: string, options: { method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'; body?: any; headers?: Record<string, string> } = {}): Promise<T> => {
    const authStore = useAuthStore()
    
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      Accept: 'application/json',
    }
    
    if (authStore.accessToken) {
      headers['Authorization'] = `Bearer ${authStore.accessToken}`
    }
    
    if (options.headers) {
      Object.assign(headers, options.headers)
    }

    try {
      const response = await $fetch<T>(`${baseURL}${url}`, {
        method: options.method || 'GET',
        headers,
        body: options.method !== 'GET' ? options.body : undefined,
      })
      return response
    } catch (error: any) {
      if (error.statusCode === 401 && authStore.refreshToken) {
        try {
          const refreshData = await $fetch<any>(`${baseURL}/auth/refresh`, {
            method: 'POST',
            body: { refresh_token: authStore.refreshToken },
          })
          // Use setTokens to properly update store (automatically persisted to cookies)
          authStore.setTokens(refreshData.access_token, refreshData.refresh_token)
          
          const retryResponse = await $fetch<T>(`${baseURL}${url}`, {
            method: options.method || 'GET',
            headers: { ...headers, Authorization: `Bearer ${refreshData.access_token}` },
            body: options.method !== 'GET' ? options.body : undefined,
          })
          return retryResponse
        } catch {
          await authStore.logout()
          throw createError({ statusCode: 401, statusMessage: 'Session expired' })
        }
      }
      throw error
    }
  }

  nuxtApp.provide('api', authApi)
})
