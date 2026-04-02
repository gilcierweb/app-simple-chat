<template>
  <div class="max-w-2xl mx-auto p-6">
    <h1 class="text-xl font-bold mb-6">{{ t('settings.title') }}</h1>

    <!-- Profile section -->
    <section class="card bg-base-100 mb-4">
      <div class="card-body">
        <h2 class="card-title text-base">{{ t('settings.profile.title') }}</h2>
        <div class="grid grid-cols-2 gap-4 mt-2">
          <div class="form-control">
            <label class="label"><span class="label-text text-xs">{{ t('settings.profile.firstName') }}</span></label>
            <input v-model="form.first_name" type="text" class="input input-sm input-bordered" :placeholder="t('settings.profile.firstNamePlaceholder')" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text text-xs">{{ t('settings.profile.lastName') }}</span></label>
            <input v-model="form.last_name" type="text" class="input input-sm input-bordered" :placeholder="t('settings.profile.lastNamePlaceholder')" />
          </div>
          <div class="form-control col-span-2">
            <label class="label"><span class="label-text text-xs">{{ t('settings.profile.nickname') }}</span></label>
            <input v-model="form.nickname" type="text" class="input input-sm input-bordered" :placeholder="t('settings.profile.nicknamePlaceholder')" />
          </div>
          <div class="form-control col-span-2">
            <label class="label"><span class="label-text text-xs">{{ t('settings.profile.bio') }}</span></label>
            <textarea v-model="form.bio" class="textarea textarea-bordered textarea-sm" rows="2" :placeholder="t('settings.profile.bioPlaceholder')"></textarea>
          </div>
        </div>
        <div class="card-actions justify-end mt-2">
          <button class="btn btn-primary btn-sm" :disabled="saving" @click="saveProfile">
            <span v-if="saving" class="loading loading-spinner loading-xs"></span>
            <span v-else>{{ t('settings.profile.save') }}</span>
          </button>
        </div>
      </div>
    </section>

    <!-- Security section -->
    <section class="card bg-base-100 mb-4">
      <div class="card-body">
        <h2 class="card-title text-base">{{ t('settings.security.title') }}</h2>

        <div class="flex items-center justify-between py-2 border-b border-base-200">
          <div>
            <p class="font-medium text-sm">{{ t('settings.security.twoFactor') }}</p>
            <p class="text-xs text-base-content/50">{{ t('settings.security.twoFactorHint') }}</p>
          </div>
          <div class="flex items-center gap-2">
            <span :class="authStore.user?.totp_enabled ? 'badge badge-success' : 'badge badge-ghost'" class="text-xs">
              {{ authStore.user?.totp_enabled ? t('settings.security.enabled') : t('settings.security.disabled') }}
            </span>
            <button class="btn btn-xs btn-ghost" @click="showTotpSetup = true">
              {{ authStore.user?.totp_enabled ? t('settings.security.manage') : t('settings.security.enable') }}
            </button>
          </div>
        </div>

        <div class="flex items-center justify-between py-2 border-b border-base-200">
          <div>
            <p class="font-medium text-sm">{{ t('settings.security.activeSessions') }}</p>
            <p class="text-xs text-base-content/50">{{ t('settings.security.activeSessionsHint') }}</p>
          </div>
          <button class="btn btn-xs btn-ghost" @click="showSessions = true">{{ t('settings.security.view') }}</button>
        </div>

        <div class="flex items-center justify-between py-2">
          <div>
            <p class="font-medium text-sm">{{ t('settings.security.encryptionKeys') }}</p>
            <p class="text-xs text-base-content/50">{{ t('settings.security.encryptionKeysHint') }}</p>
          </div>
          <button class="btn btn-xs btn-ghost" @click="showKeys = true">{{ t('settings.security.view') }}</button>
        </div>
      </div>
    </section>

    <!-- Danger zone -->
    <section class="card bg-base-100 border border-error/30">
      <div class="card-body">
        <h2 class="card-title text-base text-error">{{ t('settings.danger.title') }}</h2>
        <button class="btn btn-error btn-outline btn-sm w-fit" @click="logout">{{ t('settings.danger.signOut') }}</button>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'

definePageMeta({ 
  layout: 'default',
  requiresAuth: true,
})

const authStore = useAuthStore()
const { logout, authFetch } = useAuth()
const { t } = useI18n()

const saving = ref(false)
const showTotpSetup = ref(false)
const showSessions = ref(false)
const showKeys = ref(false)

const form = reactive({
  first_name: '',
  last_name: '',
  nickname: '',
  bio: '',
})

onMounted(async () => {
  try {
    const profile = await authFetch<any>('/profile')
    form.first_name = profile.first_name ?? ''
    form.last_name = profile.last_name ?? ''
    form.nickname = profile.nickname ?? ''
    form.bio = profile.bio ?? ''
  } catch {}
})

async function saveProfile() {
  saving.value = true
  try {
    await authFetch('/profile', { method: 'PATCH', body: JSON.stringify(form) })
  } catch {} finally {
    saving.value = false
  }
}
</script>
