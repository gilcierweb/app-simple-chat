<template>
  <div class="flex flex-col items-center justify-center h-full p-8">
    <div class="card bg-dark-900 border border-dark-800 max-w-md w-full">
      <div class="card-body items-center text-center p-12">
        <!-- Icon -->
        <div class="flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 mb-6">
          <span class="icon-[lucide--message-circle] size-10 text-primary"></span>
        </div>

        <!-- Title -->
        <h3 class="card-title text-text-primary text-xl">Select a conversation</h3>
        <p class="text-text-secondary mt-2">
          Choose an existing chat from the sidebar or start a new one
        </p>

        <!-- Action -->
        <button class="btn btn-primary btn-soft mt-6" @click="$emit('newChat')">
          <span class="icon-[lucide--plus] size-4"></span>
          Start new conversation
        </button>
      </div>
    </div>

    <!-- Security tip -->
    <div class="mt-8 flex items-center gap-2 text-xs text-text-muted">
      <span class="icon-[lucide--lock] size-4 text-primary"></span>
      <span>All messages are end-to-end encrypted</span>
    </div>
  </div>
</template>

<script setup lang="ts">
// definePageMeta({ layout: 'default' })

const authStore = useAuthStore()
const { authFetch } = useAuth()

// Load conversations on mount
onMounted(async () => {
  try {
    const data = await authFetch<any[]>('/conversations')
    const store = useConversationStore()
    store.setConversations(data)
  } catch {}
})
</script>
