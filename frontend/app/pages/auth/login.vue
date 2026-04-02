<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="text-center">
      <h3 class="font-display text-2xl font-bold text-text-primary mb-1">{{ t('auth.login.title') }}</h3>
      <p class="text-sm text-text-secondary">{{ t('auth.login.subtitle') }}</p>
    </div>

    <!-- Login Form -->
    <form @submit.prevent="submit" class="space-y-4">
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
        <div class="flex items-center justify-between mb-1.5">
          <label class="label-text text-text-secondary" for="password">
            {{ t('auth.common.password') }}
          </label>
          <NuxtLink to="/auth/forgot-password" class="link link-primary text-xs font-normal">
            {{ t('auth.login.forgotPassword') }}
          </NuxtLink>
        </div>
        <div class="input input-bordered border-dark-700 bg-dark-950 flex items-center gap-3 px-4 py-3">
          <span class="icon-[lucide--lock] size-5 text-text-muted shrink-0"></span>
          <input
            id="password"
            v-model="password"
            :type="showPassword ? 'text' : 'password'"
            autocomplete="current-password"
            :placeholder="t('auth.common.passwordPlaceholder')"
            class="bg-transparent border-0 outline-none w-full text-text-primary placeholder:text-text-muted"
            :disabled="loading"
            required
          />
          <button
            type="button"
            class="text-text-muted hover:text-primary transition-colors shrink-0"
            @click="showPassword = !showPassword"
            aria-label="Toggle password visibility"
          >
            <span v-if="!showPassword" class="icon-[lucide--eye] size-5"></span>
            <span v-else class="icon-[lucide--eye-off] size-5"></span>
          </button>
        </div>
      </div>

      <!-- TOTP Field -->
      <div v-if="needsTotp" class="form-control">
        <label class="label-text mb-1.5 block text-text-secondary" for="totp">
          {{ t('auth.login.authCode') }}
        </label>
        <div class="input input-bordered border-dark-700 bg-dark-950 flex items-center gap-3 px-4 py-3">
          <span class="icon-[lucide--shield] size-5 text-text-muted shrink-0"></span>
          <input
            id="totp"
            v-model="totpCode"
            type="text"
            inputmode="numeric"
            autocomplete="one-time-code"
            :placeholder="t('auth.login.authCodePlaceholder')"
            maxlength="6"
            class="bg-transparent border-0 outline-none w-full text-text-primary placeholder:text-text-muted text-center tracking-[0.5em] font-mono"
            :disabled="loading"
          />
        </div>
      </div>

      <!-- Remember Me -->
      <div class="flex items-center gap-2">
        <input
          id="remember"
          v-model="rememberMe"
          type="checkbox"
          class="checkbox checkbox-primary checkbox-sm"
        />
        <label class="label-text text-text-secondary text-sm cursor-pointer" for="remember">
          {{ t('auth.login.rememberMe') }}
        </label>
      </div>

      <!-- Submit Button -->
      <button
        type="submit"
        class="btn btn-lg btn-primary btn-gradient w-full shadow-lg shadow-primary/20"
        :disabled="loading"
      >
        <span v-if="loading" class="loading loading-spinner loading-sm"></span>
        <span v-else class="flex items-center justify-center gap-2">
          <span class="icon-[lucide--log-in] size-5"></span>
          {{ t('auth.login.submit') }}
        </span>
      </button>
    </form>

    <!-- Divider -->
    <div class="divider text-text-muted text-sm">{{ t('auth.common.or') }}</div>

    <!-- Register Link -->
    <div class="text-center">
      <p class="text-sm text-text-secondary">
        {{ t('auth.login.noAccount') }}
        <NuxtLink to="/auth/register" class="link link-primary font-medium">
          {{ t('auth.login.createOne') }}
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
definePageMeta({ 
  layout: 'auth',
  guestOnly: true, // Authenticated users will be redirected away
})
const { t } = useI18n()

const { login, loading } = useAuth()
const { showError, showInfo } = useApiAlert()

const email = ref('')
const password = ref('')
const totpCode = ref('')
const needsTotp = ref(false)
const showPassword = ref(false)
const rememberMe = ref(false)

async function submit() {
  try {
    await login(email.value, password.value, needsTotp.value ? totpCode.value : undefined)
  } catch (e: any) {
    const message = e?.data?.message || ''
    const errorStr = e?.data?.error || ''
    if (message.includes('2FA') || (typeof errorStr === 'string' && errorStr.includes('2FA'))) {
      needsTotp.value = true
      showInfo(t('auth.login.totpRequired'))
    } else {
      showError(e, 'auth.login.errors.invalidCredentials')
    }
  }
}
</script>

