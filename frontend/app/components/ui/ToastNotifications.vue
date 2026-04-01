<template>
  <div class="pointer-events-none fixed top-4 right-0 left-0 z-[2000] px-4 sm:left-auto sm:w-full sm:max-w-sm sm:px-0 sm:right-4">
    <TransitionGroup name="toast" tag="div" class="space-y-2">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="alert alert-soft shadow-lg pointer-events-auto items-start"
        :class="variantClass(toast.variant)"
        role="alert"
      >
        <span :class="[variantIconClass(toast.variant), 'size-5 mt-0.5 shrink-0']"></span>
        <span class="break-words text-sm leading-5">{{ toast.message }}</span>
        <button
          v-if="toast.closable"
          type="button"
          class="btn btn-text btn-circle btn-xs ms-2"
          :aria-label="t('toast.close')"
          @click="remove(toast.id)"
        >
          <span class="icon-[tabler--x] size-3.5"></span>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import type { ToastVariant } from '~/composables/useToast'

const { toasts, remove } = useToast()
const { t } = useI18n()

function variantClass(variant: ToastVariant) {
  return {
    primary: 'alert-primary',
    secondary: 'alert-secondary',
    info: 'alert-info',
    success: 'alert-success',
    warning: 'alert-warning',
    error: 'alert-error',
  }[variant]
}

function variantIconClass(variant: ToastVariant) {
  return {
    primary: 'icon-[tabler--sparkles]',
    secondary: 'icon-[tabler--layers-intersect]',
    info: 'icon-[tabler--info-circle]',
    success: 'icon-[tabler--circle-check]',
    warning: 'icon-[tabler--alert-triangle]',
    error: 'icon-[tabler--alert-circle]',
  }[variant]
}
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.22s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
}
</style>
