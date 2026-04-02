/**
 * auth-init.client.ts
 *
 * This client-side plugin manages the lifecycle of E2EE keys and WebSocket connections
 * based on the user's authentication state.
 */

export default defineNuxtPlugin((_nuxtApp) => {
  const authStore = useAuthStore()
  const keyStore = useKeyStore()
  const ws = useWebSocket()

  // Watch for authentication status changes
  // We use a watcher to react to login/logout/refresh events
  watch(
    () => authStore.isAuthenticated,
    async (isAuth) => {
      // Check if we have an access token and user is authenticated
      if (isAuth && authStore.accessToken) {
        console.group('[AuthInit] Initializing secure context')
        console.info('User is authenticated, preparing E2EE & WS...')

        // 1. Ensure E2EE keys are local and synced with server
        // This is non-blocking to keep the initial load fast
        keyStore.ensureKeys(authStore.accessToken)
          .then(() => console.info('[AuthInit] E2E Key Setup complete'))
          .catch((err) => {
            console.error('[AuthInit] E2E Key Setup failed:', err)
          })

        // 2. Connect to WebSocket for real-time traffic
        // WebSocket logic in useWebSocket handles reconnection automatically
        ws.connect()
        
        console.groupEnd()
      } else {
        // When user logs out or session expires
        console.info('[AuthInit] Secure context teardown: disconnecting WS')
        ws.disconnect()
      }
    },
    { immediate: true } // Run immediately on mount to catch existing sessions
  )
})
