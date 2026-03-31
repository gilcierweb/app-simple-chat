<template>
  <div>
    <!-- Header -->
    <div class="text-center mb-6">
      <h2 class="font-display text-2xl font-bold text-text-primary">Create account</h2>
      <p class="text-sm text-text-secondary mt-1">Join the secure messaging network</p>
    </div>

    <!-- Success Alert -->
    <div v-if="success" class="alert alert-soft alert-success mb-4">
      <span class="icon-[lucide--check-circle] size-5"></span>
      <div>
        <p class="font-medium text-sm">Account created successfully!</p>
        <p class="text-xs opacity-80">Check your email to confirm your account.</p>
      </div>
    </div>

    <!-- Error Alert -->
    <div v-if="error" class="alert alert-soft alert-error mb-4">
      <span class="icon-[lucide--circle-x] size-5"></span>
      <span class="text-sm">{{ error }}</span>
    </div>

    <!-- Register Form -->
    <form v-if="!success" @submit.prevent="submit" class="space-y-4">
      <!-- Email Input -->
      <div class="form-control">
        <label class="label">
          <span class="label-text text-text-secondary">Email</span>
        </label>
        <label class="input-group">
          <span class="input-group-text">
            <span class="icon-[lucide--mail] size-5 text-text-muted"></span>
          </span>
          <input
            v-model="email"
            type="email"
            autocomplete="email"
            placeholder="you@example.com"
            class="input input-filled flex-1"
            :disabled="loading"
            required
          />
        </label>
      </div>

      <!-- Password Input -->
      <div class="form-control">
        <label class="label">
          <span class="label-text text-text-secondary">Password</span>
        </label>
        <label class="input-group">
          <span class="input-group-text">
            <span class="icon-[lucide--lock] size-5 text-text-muted"></span>
          </span>
          <input
            v-model="password"
            type="password"
            autocomplete="new-password"
            placeholder="Min. 8 characters"
            class="input input-filled flex-1"
            :disabled="loading"
            minlength="8"
            required
          />
        </label>
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
        <label class="label">
          <span class="label-text text-text-secondary">Confirm password</span>
        </label>
        <label class="input-group">
          <span class="input-group-text">
            <span class="icon-[lucide--lock-keyhole] size-5 text-text-muted"></span>
          </span>
          <input
            v-model="confirmPassword"
            type="password"
            autocomplete="new-password"
            placeholder="Repeat password"
            class="input input-filled flex-1"
            :class="{ 'input-error': confirmPassword && confirmPassword !== password }"
            :disabled="loading"
            required
          />
        </label>
        <p v-if="confirmPassword && confirmPassword !== password" class="text-xs text-error mt-1 flex items-center gap-1">
          <span class="icon-[lucide--alert-circle] size-3"></span>
          Passwords do not match
        </p>
      </div>

      <!-- Submit Button -->
      <button
        type="submit"
        class="btn btn-primary w-full"
        :disabled="loading || (!!confirmPassword && confirmPassword !== password)"
      >
        <span v-if="loading" class="loading loading-spinner loading-sm"></span>
        <span v-else class="flex items-center gap-2">
          <span class="icon-[lucide--user-plus] size-5"></span>
          Create account
        </span>
      </button>
    </form>

    <!-- Divider -->
    <div class="divider text-text-muted text-sm my-6">or</div>

    <!-- Login Link -->
    <div class="text-center">
      <p class="text-sm text-text-secondary">
        Already have an account?
        <NuxtLink to="/auth/login" class="link link-primary font-medium">
          Sign in
        </NuxtLink>
      </p>
    </div>

    <!-- Security Badge -->
    <div class="mt-6 flex items-center justify-center gap-2 text-xs text-text-muted">
      <span class="icon-[lucide--shield-check] size-4 text-primary"></span>
      <span>End-to-end encrypted</span>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'auth' })

const { register, loading, error } = useAuth()

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
  return ['Too short', 'Weak', 'Fair', 'Good', 'Strong'][strength.value]
})

function strengthColor(n: number) {
  if (n > strength.value) return 'bg-dark-700'
  const colors = ['', 'bg-error', 'bg-warning', 'bg-brand-500', 'bg-primary']
  return colors[strength.value] || 'bg-dark-700'
}

async function submit() {
  if (password.value !== confirmPassword.value) return
  try {
    await register(email.value, password.value)
    success.value = true
  } catch {}
}
</script>
