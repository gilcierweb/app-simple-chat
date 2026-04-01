<template>
  <Teleport to="body">
    <div class="fixed top-4 right-4 z-[9999] flex flex-col gap-3 w-full max-w-sm pointer-events-none">
      <TransitionGroup
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 translate-x-4"
        enter-to-class="opacity-100 translate-x-0"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 translate-x-0"
        leave-to-class="opacity-0 translate-x-4"
        move-class="transition-transform duration-300"
      >
        <div
          v-for="alert in alerts"
          :key="alert.id"
          class="pointer-events-auto"
        >
          <UiAlert
            :variant="alert.variant"
            :color="alert.color"
            :icon="alert.icon"
            :title="alert.title"
            :message="alert.message"
            :dismissible="alert.dismissible"
            @dismiss="remove(alert.id)"
          />
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import UiAlert from './Alert.vue'

const { alerts, remove } = useAlert()
</script>
