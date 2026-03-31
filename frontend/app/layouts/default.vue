<template>
  <div class="chat-layout bg-dark-950">
    <!-- Sidebar -->
    <aside class="flex flex-col h-dvh border-r border-dark-800 bg-dark-900">
      <!-- Header -->
      <div class="flex items-center justify-between px-4 py-3 border-b border-dark-800">
        <div class="flex items-center gap-3">
          <div class="avatar avatar-placeholder">
            <div class="w-9 rounded-full bg-primary text-dark-950 font-bold text-sm flex items-center justify-center">
              {{ userInitial }}
            </div>
          </div>
          <span class="font-semibold text-text-primary truncate max-w-[140px]">
            {{ authStore.profile?.nickname || authStore.user?.email?.split('@')[0] }}
          </span>
        </div>
        <div class="flex items-center gap-1">
          <button class="btn btn-ghost btn-sm btn-square" @click="showNewChat = true" title="New conversation">
            <span class="icon-[lucide--square-pen] size-5"></span>
          </button>
          <NuxtLink to="/settings" class="btn btn-ghost btn-sm btn-square" title="Settings">
            <span class="icon-[lucide--settings] size-5"></span>
          </NuxtLink>
        </div>
      </div>

      <!-- Search -->
      <div class="px-3 py-2">
        <label class="input-group input-group-sm">
          <span class="input-group-text">
            <span class="icon-[lucide--search] size-4 text-text-muted"></span>
          </span>
          <input
            v-model="search"
            type="text"
            placeholder="Search conversations..."
            class="input input-filled flex-1"
          />
        </label>
      </div>

      <!-- Conversation list -->
      <nav class="flex-1 overflow-y-auto">
        <ConversationList :search="search" />
      </nav>
    </aside>

    <!-- Main area -->
    <main class="flex flex-col h-dvh overflow-hidden bg-dark-950">
      <slot />
    </main>

    <!-- New chat modal -->
    <NewChatModal v-if="showNewChat" @close="showNewChat = false" />
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'

const authStore = useAuthStore()
const showNewChat = ref(false)
const search = ref('')

const userInitial = computed(() => {
  const name = authStore.profile?.nickname || authStore.user?.email || '?'
  return name[0].toUpperCase()
})

// Connect WebSocket on mount
const ws = useWebSocket()
onMounted(() => ws.connect())
onUnmounted(() => ws.disconnect())
</script>
