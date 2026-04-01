<template>
  <div
    :class="alertClasses"
    role="alert"
  >
    <!-- Icon -->
    <span v-if="icon || $slots.icon" :class="iconClasses">
      <slot name="icon">
        <span :class="icon"></span>
      </slot>
    </span>

    <!-- Content -->
    <div class="flex-1 min-w-0">
      <slot>
        <p v-if="title" class="font-semibold">{{ title }}</p>
        <p v-if="message" class="text-sm">{{ message }}</p>
      </slot>
    </div>

    <!-- Dismiss Button -->
    <button
      v-if="dismissible"
      type="button"
      class="ms-auto cursor-pointer leading-none text-current opacity-70 hover:opacity-100 transition-opacity shrink-0"
      :aria-label="dismissLabel || 'Close'"
      @click="$emit('dismiss')"
    >
      <span class="icon-[lucide--x] size-5"></span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'solid' | 'soft' | 'outline' | 'dashed'
  color?: 'primary' | 'secondary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  icon?: string
  title?: string
  message?: string
  dismissible?: boolean
  dismissLabel?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'soft',
  color: 'primary'
})

defineEmits<{
  dismiss: []
}>()

const alertClasses = computed(() => {
  const base = 'alert flex items-start gap-3 p-4 rounded-lg'
  const variants: Record<string, string> = {
    solid: '',
    soft: 'alert-soft',
    outline: 'alert-outline',
    dashed: 'alert-outline border-dashed'
  }
  const colors: Record<string, string> = {
    primary: 'alert-primary',
    secondary: 'alert-secondary',
    info: 'alert-info',
    success: 'alert-success',
    warning: 'alert-warning',
    error: 'alert-error',
    neutral: ''
  }

  return [base, variants[props.variant], colors[props.color]].filter(Boolean).join(' ')
})

const iconClasses = computed(() => {
  return 'shrink-0 size-5 mt-0.5'
})
</script>
