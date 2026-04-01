<template>
  <div>
    <!-- Header -->
    <div class="text-center mb-6">
      <h2 class="font-display text-2xl font-bold text-text-primary">{{ t('auth.login.title') }}</h2>
      <p class="text-sm text-text-secondary mt-1">{{ t('auth.login.subtitle') }}</p>
    </div>

    <!-- Error Alert -->
    <div v-if="error" class="alert alert-soft alert-error mb-4">
      <span class="icon-[lucide--circle-x] size-5"></span>
      <span class="text-sm">{{ error }}</span>
    </div>

    <!-- Login Form -->
    <form @submit.prevent="submit" class="space-y-4">
      <!-- Email Input -->
      <div class="form-control">
        <label class="label">
          <span class="label-text text-text-secondary">{{ t('auth.common.email') }}</span>
        </label>
        <label class="input-group">
          <span class="input-group-text">
            <span class="icon-[lucide--mail] size-5 text-text-muted"></span>
          </span>
          <input
            v-model="email"
            type="email"
            autocomplete="email"
            :placeholder="t('auth.common.emailPlaceholder')"
            class="input input-filled flex-1"
            :disabled="loading"
            required
          />
        </label>
      </div>

      <!-- Password Input -->
      <div class="form-control">
        <label class="label">
          <span class="label-text text-text-secondary">{{ t('auth.common.password') }}</span>
          <NuxtLink to="/auth/forgot-password" class="label-text-alt link link-primary text-xs">
            {{ t('auth.login.forgotPassword') }}
          </NuxtLink>
        </label>
        <label class="input-group">
          <span class="input-group-text">
            <span class="icon-[lucide--lock] size-5 text-text-muted"></span>
          </span>
          <input
            v-model="password"
            :type="showPassword ? 'text' : 'password'"
            autocomplete="current-password"
            :placeholder="t('auth.common.passwordPlaceholder')"
            class="input input-filled flex-1"
            :disabled="loading"
            required
          />
          <span
            class="input-group-text cursor-pointer hover:text-primary"
            @click="showPassword = !showPassword"
          >
            <span v-if="!showPassword" class="icon-[lucide--eye] size-5"></span>
            <span v-else class="icon-[lucide--eye-off] size-5"></span>
          </span>
        </label>
      </div>

      <!-- TOTP Field -->
      <div v-if="needsTotp" class="form-control">
        <label class="label">
          <span class="label-text text-text-secondary">{{ t('auth.login.authCode') }}</span>
        </label>
        <label class="input-group">
          <span class="input-group-text">
            <span class="icon-[lucide--shield] size-5 text-text-muted"></span>
          </span>
          <input
            v-model="totpCode"
            type="text"
            inputmode="numeric"
            autocomplete="one-time-code"
            :placeholder="t('auth.login.authCodePlaceholder')"
            maxlength="6"
            class="input input-filled flex-1 text-center tracking-[0.5em] font-mono"
            :disabled="loading"
          />
        </label>
      </div>

      <!-- Submit Button -->
      <button type="submit" class="btn btn-primary w-full" :disabled="loading">
        <span v-if="loading" class="loading loading-spinner loading-sm"></span>
        <span v-else class="flex items-center gap-2">
          <span class="icon-[lucide--log-in] size-5"></span>
          {{ t('auth.login.submit') }}
        </span>
      </button>
    </form>

    <!-- Divider -->
    <div class="divider text-text-muted text-sm my-6">{{ t('auth.common.or') }}</div>

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
    <div class="mt-6 flex items-center justify-center gap-2 text-xs text-text-muted">
      <span class="icon-[lucide--shield-check] size-4 text-primary"></span>
      <span>{{ t('auth.common.e2eBadge') }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'auth' })
const { t } = useI18n()

const { login, loading, error } = useAuth()

const email = ref('')
const password = ref('')
const totpCode = ref('')
const needsTotp = ref(false)
const showPassword = ref(false)

async function submit() {
  try {
    await login(email.value, password.value, needsTotp.value ? totpCode.value : undefined)
  } catch (e: any) {
    if (e?.data?.message?.includes('2FA')) {
      needsTotp.value = true
    }
  }
}
</script>
