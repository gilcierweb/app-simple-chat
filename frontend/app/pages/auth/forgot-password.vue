<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="text-center">
      <h3 class="font-display text-2xl font-bold text-text-primary mb-1">{{ t('auth.forgotPassword.title') }}</h3>
      <p class="text-sm text-text-secondary">{{ t('auth.forgotPassword.subtitle') }}</p>
    </div>

    <!-- Forgot Password Form -->
    <form v-if="!success" @submit.prevent="submit" class="space-y-4">
      <!-- Email Input -->
      <div class="form-control">
        <label class="label-text mb-1.5 block text-text-secondary" for="email">
          {{ t('auth.common.email') }}
        </label>
        <div class="input input-bordered border-dark-700 bg-dark-950 flex items-center gap-3 px-4 py-3">
          <span class="icon-[lucide--mail] size-5 text-text-muted shrink-0"></span>
          <input
            id="email"
            v-model="email"
            type="email"
            autocomplete="email"
            :placeholder="t('auth.common.emailPlaceholder')"
            class="bg-transparent border-0 outline-none w-full text-text-primary placeholder:text-text-muted"
            :disabled="loading"
            required
          />
        </div>
      </div>

      <!-- Submit Button -->
      <button
        type="submit"
        class="btn btn-lg btn-primary btn-gradient w-full shadow-lg shadow-primary/20"
        :disabled="loading"
      >
        <span v-if="loading" class="loading loading-spinner loading-sm"></span>
        <span v-else class="flex items-center justify-center gap-2">
          <span class="icon-[lucide--send] size-5"></span>
          {{ t('auth.forgotPassword.submit') }}
        </span>
      </button>
    </form>

    <!-- Divider -->
    <div class="divider text-text-muted text-sm">{{ t('auth.common.or') }}</div>

    <!-- Back to Login -->
    <div class="text-center">
      <NuxtLink to="/auth/login" class="link link-primary text-sm font-medium flex items-center justify-center gap-2">
        <span class="icon-[lucide--arrow-left] size-4"></span>
        {{ t('auth.forgotPassword.backToLogin') }}
      </NuxtLink>
    </div>

    <!-- Security Badge -->
    <div class="flex items-center justify-center gap-2 text-xs text-text-muted pt-2">
      <span class="icon-[lucide--shield-check] size-4 text-primary"></span>
      <span>{{ t('auth.common.e2eBadge') }}</span>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useApiAlert } from '~/composables/useApiAlert'

definePageMeta({ layout: 'auth' })
const { t } = useI18n()

const { showError, showSuccess } = useApiAlert()

const email = ref('')
const success = ref(false)
const loading = ref(false)

async function submit() {
  loading.value = true
  try {
    // TODO: Implementar chamada real de forgotPassword na API
    await new Promise(resolve => setTimeout(resolve, 1500))
    success.value = true
    showSuccess(`${t('auth.forgotPassword.successTitle')} ${t('auth.forgotPassword.successHint')}`)
  } catch (e: any) {
    showError(e)
  } finally {
    loading.value = false
  }
}
</script>