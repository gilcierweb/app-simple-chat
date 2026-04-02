// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  srcDir: 'app',

  vite: {
    plugins: [
      tailwindcss(),
    ],
    optimizeDeps: {
      include: [
        'flyonui/flyonui', // CJS
      ]
    }

  },

  modules: ['@pinia/nuxt', 'pinia-plugin-persistedstate/nuxt', '@nuxtjs/i18n'],

  app: {
    head: {
      title: 'Simple Chat - Simple Chat',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'Secure end-to-end encrypted chat' },
        { name: 'robots', content: 'noindex, nofollow' }, // keep off search engines
        { name: 'theme-color', content: '#7C3AED' },
      ],
      link: [
        { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' },
        {
          rel: 'preconnect',
          href: 'https://fonts.googleapis.com',
        },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=Space+Grotesk:wght@400;500;600;700&display=swap',
        },
      ],
    },
    pageTransition: { name: 'page', mode: 'out-in' },
  },
  
  // -- Runtime Config
    runtimeConfig: {
      // Server-only (private)
      apiSecret: '',
      // Public (exposed to client)
      public: {
        // @ts-ignore
        apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://localhost:8080/api/v1',
        // @ts-ignore
        wsUrl: process.env.NUXT_PUBLIC_WS_URL || 'ws://localhost:8080/api/v1/ws',
        // @ts-ignore
        cdnUrl: process.env.NUXT_PUBLIC_CDN_URL || 'https://cdn.simple-chat.com',
        // @ts-ignore
        stripeKey: process.env.NUXT_PUBLIC_STRIPE_KEY || '',
        // @ts-ignore
        appName: process.env.NUXT_PUBLIC_APP_NAME || 'Simple Chat',
      },
    },
  
    i18n: {
      locales: [
        {
          code: 'en',
          iso: 'en-US',
          file: 'en.json',
          name: 'English'
        },
        {
          code: 'es',
          iso: 'es-ES',
          file: 'es.json',
          name: 'Español'
        },
        {
          code: 'pt',
          iso: 'pt-BR',
          file: 'pt-BR.json',
          name: 'Português'
        },
        {
          code: 'pt-BR',
          iso: 'pt-BR',
          file: 'pt-BR.json',
          name: 'Português Brasil'
        }
      ],
      defaultLocale: 'pt-BR',
      strategy: 'prefix_except_default',
      lazy: true,
      detectBrowserLanguage: {
        useCookie: true,
        cookieKey: 'i18n_redirected',
        redirectOn: 'root'
      }
    },
  
    piniaPluginPersistedstate: {
      storage: 'cookies',
      cookieOptions: {
        sameSite: 'lax',
        maxAge: 7 * 24 * 60 * 60, // 7 days
      },
    },
  
    nitro: {
      compressPublicAssets: true,
    },

})