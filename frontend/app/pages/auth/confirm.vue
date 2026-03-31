<template>
  <div class="text-center">
    <div v-if="loading" class="py-4">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <p class="mt-3 text-sm text-base-content/60">Confirming your email...</p>
    </div>

    <div v-else-if="success" class="py-4">
      <div class="text-5xl mb-4">✅</div>
      <h2 class="text-xl font-bold mb-2">Email confirmed!</h2>
      <p class="text-sm text-base-content/60 mb-6">Your account is ready. You can now sign in.</p>
      <NuxtLink to="/auth/login" class="btn btn-primary">Sign in</NuxtLink>
    </div>

    <div v-else class="py-4">
      <div class="text-5xl mb-4">❌</div>
      <h2 class="text-xl font-bold mb-2">Confirmation failed</h2>
      <p class="text-sm text-base-content/60 mb-6">{{ error }}</p>
      <NuxtLink to="/auth/login" class="btn btn-ghost btn-sm">Back to login</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'auth' })

const route = useRoute()
const config = useRuntimeConfig()

const loading = ref(true)
const success = ref(false)
const error = ref('')

onMounted(async () => {
  const token = route.query.token as string
  if (!token) {
    error.value = 'Missing confirmation token'
    loading.value = false
    return
  }
  try {
    await $fetch(`${config.public.apiBaseUrl}/auth/confirm/${token}`)
    success.value = true
  } catch (e: any) {
    error.value = e?.data?.message || 'Invalid or expired confirmation link'
  } finally {
    loading.value = false
  }
})
</script>
