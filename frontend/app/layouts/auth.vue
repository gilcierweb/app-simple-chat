<template>
  <div class="min-h-screen bg-dark-950 flex items-center justify-center p-4">
    <div class="w-full max-w-md">
      <!-- Logo -->
      <div class="flex items-center justify-center gap-2 mb-8">
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-primary text-dark-950">
          <span class="icon-[lucide--message-circle] size-7"></span>
        </div>
        <span class="font-display text-2xl font-bold text-text-primary">AppSimpleChat</span>
      </div>

      <!-- Auth Card -->
      <div class="card bg-dark-900 border border-dark-800 shadow-xl">
        <div class="card-body p-8">
          <slot />
        </div>
      </div>

      <!-- Footer -->
      <p class="text-center text-sm text-text-muted mt-6">
        {{ t('layout.auth.footer') }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
</script>
