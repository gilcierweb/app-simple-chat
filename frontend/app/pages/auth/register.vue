<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="text-center">
      <h3 class="font-display text-2xl font-bold text-text-primary mb-1">{{ t('auth.register.title') }}</h3>
      <p class="text-sm text-text-secondary">{{ t('auth.register.subtitle') }}</p>
    </div>

    <!-- Register Form -->
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

      <!-- Password Input -->
      <div class="form-control">
        <label class="label-text mb-1.5 block text-text-secondary" for="password">
          {{ t('auth.common.password') }}
        </label>
        <div class="input input-bordered border-dark-700 bg-dark-950 flex items-center gap-3 px-4 py-3">
          <span class="icon-[lucide--lock] size-5 text-text-muted shrink-0"></span>
          <input
            id="password"
            v-model="password"
            type="password"
            autocomplete="new-password"
            :placeholder="t('auth.register.passwordPlaceholder')"
            class="bg-transparent border-0 outline-none w-full text-text-primary placeholder:text-text-muted"
            :disabled="loading"
            minlength="8"
            required
          />
        </div>
        <!-- Strength indicator -->
        <div class="flex gap-1 mt-2">
          <div
            v-for="n in 4"
            :key="n"
            class="h-1 flex-1 rounded-full transition-colors duration-300"
            :class="strengthColor(n)"
          ></div>
        </div>
        <p class="text-xs text-text-muted mt-1">{{ strengthLabel }}</p>
      </div>

      <!-- Confirm Password Input -->
      <div class="form-control">
        <label class="label-text mb-1.5 block text-text-secondary" for="confirmPassword">
          {{ t('auth.register.confirmPassword') }}
        </label>
        <div
          class="input input-bordered flex items-center gap-3 px-4 py-3"
          :class="confirmPassword && confirmPassword !== password ? 'border-error bg-error/5' : 'border-dark-700 bg-dark-950'"
        >
          <span class="icon-[lucide--lock-keyhole] size-5 text-text-muted shrink-0"></span>
          <input
            id="confirmPassword"
            v-model="confirmPassword"
            type="password"
            autocomplete="new-password"
            :placeholder="t('auth.register.confirmPasswordPlaceholder')"
            class="bg-transparent border-0 outline-none w-full text-text-primary placeholder:text-text-muted"
            :disabled="loading"
            required
          />
        </div>
        <p v-if="confirmPassword && confirmPassword !== password" class="text-xs text-error mt-1 flex items-center gap-1">
          <span class="icon-[lucide--alert-circle] size-3"></span>
          {{ t('auth.register.passwordMismatch') }}
        </p>
      </div>

      <!-- Submit Button -->
      <button
        type="submit"
        class="btn btn-lg btn-primary btn-gradient w-full shadow-lg shadow-primary/20"
        :disabled="loading || (!!confirmPassword && confirmPassword !== password)"
      >
        <span v-if="loading" class="loading loading-spinner loading-sm"></span>
        <span v-else class="flex items-center justify-center gap-2">
          <span class="icon-[lucide--user-plus] size-5"></span>
          {{ t('auth.register.submit') }}
        </span>
      </button>
    </form>

    <!-- Divider -->
    <div class="divider text-text-muted text-sm">{{ t('auth.common.or') }}</div>

    <!-- Login Link -->
    <div class="text-center">
      <p class="text-sm text-text-secondary">
        {{ t('auth.register.hasAccount') }}
        <NuxtLink to="/auth/login" class="link link-primary font-medium">
          {{ t('auth.register.signIn') }}
        </NuxtLink>
      </p>
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

const { register, loading } = useAuth()
const { showError, showSuccess } = useApiAlert()

const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const success = ref(false)

const strength = computed(() => {
  const p = password.value
  if (!p) return 0
  let s = 0
  if (p.length >= 8) s++
  if (p.length >= 12) s++
  if (/[A-Z]/.test(p) && /[0-9]/.test(p)) s++
  if (/[^A-Za-z0-9]/.test(p)) s++
  return s
})

const strengthLabel = computed(() => {
  if (!password.value) return ''
  return [
    t('auth.register.strength.tooShort'),
    t('auth.register.strength.weak'),
    t('auth.register.strength.fair'),
    t('auth.register.strength.good'),
    t('auth.register.strength.strong'),
  ][strength.value]
})

function strengthColor(n: number) {
  if (n > strength.value) return 'bg-dark-700'
  const colors = ['', 'bg-error', 'bg-warning', 'bg-brand-500', 'bg-primary']
  return colors[strength.value] || 'bg-dark-700'
}

async function submit() {
  if (password.value !== confirmPassword.value) {
    showError({ message: t('auth.register.passwordMismatch') })
    return
  }
  try {
    await register(email.value, password.value)
    success.value = true
    showSuccess(`${t('auth.register.successTitle')} ${t('auth.register.successHint')}`)
  } catch (e: any) {
    showError(e, 'auth.register.errors.generic')
  }
}
</script>
