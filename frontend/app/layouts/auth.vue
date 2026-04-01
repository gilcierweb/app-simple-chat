<template>
  <div class="min-h-screen flex items-center justify-center overflow-x-hidden bg-dark-950 py-10 px-4 sm:px-6 lg:px-8 relative">
    <!-- Background decoration -->
    <div class="absolute inset-0 overflow-hidden pointer-events-none">
      <div class="absolute top-0 left-1/4 w-96 h-96 bg-primary/5 rounded-full blur-3xl"></div>
      <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-brand-500/5 rounded-full blur-3xl"></div>
    </div>

    <div class="relative z-10 w-full max-w-md">
      <!-- Main Card -->
      <div class="card bg-dark-900 border border-dark-800 shadow-2xl shadow-primary/5">
        <div class="card-body p-8 space-y-6">
          <!-- Logo Header -->
          <div class="flex items-center gap-3 justify-center">
            <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-primary to-brand-600 text-dark-950 shadow-lg shadow-primary/20">
              <span class="icon-[lucide--message-circle] size-7"></span>
            </div>
            <div>
              <h2 class="font-display text-xl font-bold text-text-primary">AppSimpleChat</h2>
              <p class="text-xs text-text-muted">Conversas seguras e privadas</p>
            </div>
          </div>

          <!-- Divider -->
          <div class="divider divider-neutral text-text-muted text-xs my-2">{{ t('layout.auth.secureConnection') }}</div>

          <!-- Page Content -->
          <slot />

          <!-- Footer -->
          <div class="text-center pt-2">
            <p class="text-xs text-text-muted">
              {{ t('layout.auth.footer') }}
            </p>
          </div>
        </div>
      </div>

      <!-- Global Alert Container for Auth Pages -->
      <AlertContainer />
    </div>
  </div>
</template>

<script setup lang="ts">
import AlertContainer from '~/components/ui/AlertContainer.vue'

const { t } = useI18n()
</script>
