<template>
  <div class="text-center">
    <div v-if="loading" class="py-4">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <p class="mt-3 text-sm text-base-content/60">{{ t('auth.confirm.loading') }}</p>
    </div>

    <div v-else-if="success" class="py-4">
      <div class="text-5xl mb-4">✅</div>
      <h2 class="text-xl font-bold mb-2">{{ t('auth.confirm.successTitle') }}</h2>
      <p class="text-sm text-base-content/60 mb-6">{{ t('auth.confirm.successHint') }}</p>
      <NuxtLink to="/auth/login" class="btn btn-primary">{{ t('auth.confirm.signIn') }}</NuxtLink>
    </div>

    <div v-else class="py-4">
      <div class="text-5xl mb-4">❌</div>
      <h2 class="text-xl font-bold mb-2">{{ t('auth.confirm.errorTitle') }}</h2>
      <p class="text-sm text-base-content/60 mb-6">{{ error }}</p>
      <NuxtLink to="/auth/login" class="btn btn-ghost btn-sm">{{ t('auth.confirm.backToLogin') }}</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'auth' })
const { t } = useI18n()

const route = useRoute()
const config = useRuntimeConfig()

const loading = ref(true)
const success = ref(false)
const error = ref('')

onMounted(async () => {
  const token = route.query.token as string
  if (!token) {
    error.value = t('auth.confirm.missingToken')
    loading.value = false
    return
  }
  try {
    await $fetch(`${config.public.apiBase}/auth/confirm/${token}`)
    success.value = true
  } catch (e: any) {
    error.value = e?.data?.message || t('auth.confirm.invalidLink')
  } finally {
    loading.value = false
  }
})
</script>
