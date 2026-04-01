<template>
  <div class="h-dvh flex bg-dark-950">
    <!-- Mobile Toggle Button -->
    <button
      type="button"
      class="btn btn-text btn-square sm:hidden fixed top-3 left-3 z-50 bg-dark-900/80 backdrop-blur"
      aria-haspopup="dialog"
      aria-expanded="false"
      aria-controls="collapsible-mini-sidebar"
      @click="toggleMobileSidebar"
    >
      <span class="icon-[tabler--menu-2] size-5"></span>
    </button>

    <!-- Sidebar -->
    <aside
      id="collapsible-mini-sidebar"
      ref="sidebarRef"
      class="overlay [--auto-close:sm] overlay-minified:w-17 sm:shadow-none overlay-open:translate-x-0 drawer drawer-start w-72 sm:relative sm:z-0 sm:flex sm:translate-x-0 border-e border-dark-800 bg-dark-900 transition-all duration-300"
      :class="{ 'overlay-minified w-17': isCollapsed }"
      role="dialog"
      tabindex="-1"
    >
      <!-- Header -->
      <div class="drawer-header py-3 px-4 w-full flex items-center justify-between border-b border-dark-800">
        <div class="flex items-center gap-3" :class="{ 'hidden': isCollapsed }">
          <div class="avatar avatar-placeholder">
            <div class="w-9 rounded-full bg-primary text-dark-950 font-bold text-sm flex items-center justify-center">
              {{ userInitial }}
            </div>
          </div>
          <span class="font-semibold text-text-primary truncate max-w-[140px]">
            {{ authStore.profile?.nickname || authStore.user?.email?.split('@')[0] }}
          </span>
        </div>

        <!-- Toggle Button (Desktop) -->
        <button
          type="button"
          class="btn btn-ghost btn-circle btn-sm"
          :aria-label="t('layout.default.toggleNavigation')"
          @click="toggleSidebar"
        >
          <span class="icon-[tabler--menu-2] size-5"></span>
        </button>
      </div>

      <!-- Search (hidden when collapsed) -->
      <div v-if="!isCollapsed" class="px-3 py-2">
        <label class="input-group input-group-sm">
          <span class="input-group-text">
            <span class="icon-[tabler--search] size-4 text-base-content/50"></span>
          </span>
          <input
            v-model="search"
            type="text"
            :placeholder="t('layout.default.searchPlaceholder')"
            class="input input-filled flex-1 bg-dark-800 border-dark-700"
          />
        </label>
      </div>

      <!-- New Chat Button (icon only when collapsed) -->
      <div class="px-3 py-2">
        <button
          class="btn btn-primary btn-sm w-full"
          :class="{ 'btn-square': isCollapsed }"
          @click="convStore.openNewChatModal()"
        >
          <span class="icon-[tabler--message-circle-plus] size-4"></span>
          <span :class="{ 'hidden': isCollapsed }">{{ t('layout.default.newChat') }}</span>
        </button>
      </div>

      <!-- Conversation List -->
      <div class="flex-1 overflow-y-auto px-2 py-2">
        <div v-if="isCollapsed" class="space-y-1">
          <!-- Collapsed: Show conversation avatars only -->
          <NuxtLink
            v-for="conv in convStore.conversations.slice(0, 8)"
            :key="conv.id"
            :to="`/conversations/${conv.id}`"
            class="btn btn-ghost btn-circle btn-sm mx-auto block"
            :class="{ 'bg-primary/20': activeId === conv.id }"
            :title="convName(conv)"
            @click="convStore.setActiveConversation(conv.id)"
          >
            <div class="w-8 h-8 rounded-full bg-brand-600 text-white flex items-center justify-center text-xs font-bold">
              {{ convInitial(conv) }}
            </div>
          </NuxtLink>
        </div>
        <div v-else>
          <ConversationList :search="search" />
        </div>
      </div>

      <!-- Footer Menu -->
      <div class="border-t border-dark-800 p-2">
        <ul class="menu p-0 gap-1">
          <li>
            <NuxtLink to="/settings" class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-dark-800">
              <span class="icon-[tabler--settings] size-5"></span>
              <span :class="{ 'hidden': isCollapsed }">{{ t('layout.default.settings') }}</span>
            </NuxtLink>
          </li>
          <li>
            <button
              class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-dark-800 w-full text-left text-error"
              @click="handleLogout"
            >
              <span class="icon-[tabler--logout-2] size-5"></span>
              <span :class="{ 'hidden': isCollapsed }">{{ t('layout.default.logout') }}</span>
            </button>
          </li>
        </ul>
      </div>
    </aside>

    <!-- Overlay for mobile -->
    <div
      v-if="mobileOpen"
      class="fixed inset-0 bg-black/50 z-30 sm:hidden"
      @click="closeMobileSidebar"
    ></div>

    <!-- Main area -->
    <main class="flex-1 flex flex-col h-full min-h-0 overflow-hidden bg-dark-950">
      <slot />
    </main>

    <!-- New chat modal -->
    <NewChatModal v-if="convStore.showNewChatModal" @close="convStore.closeNewChatModal()" />
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'
import { useConversationStore } from '~/stores/conversations'
import ConversationList from '~/components/chat/ConversationList.vue'
import NewChatModal from '~/components/chat/NewChatModal.vue'
import type { Conversation } from '~/types'

const authStore = useAuthStore()
const convStore = useConversationStore()
const { t } = useI18n()
const search = ref('')
const isCollapsed = ref(false)
const mobileOpen = ref(false)
const sidebarRef = ref<HTMLElement>()

const activeId = computed(() => convStore.activeConversationId)

const userInitial = computed(() => {
  const name = authStore.profile?.nickname || authStore.user?.email || '?'
  return name[0].toUpperCase()
})

function toggleSidebar() {
  isCollapsed.value = !isCollapsed.value
}

function toggleMobileSidebar() {
  mobileOpen.value = !mobileOpen.value
  if (mobileOpen.value && sidebarRef.value) {
    sidebarRef.value.classList.add('overlay-open')
  } else if (sidebarRef.value) {
    sidebarRef.value.classList.remove('overlay-open')
  }
}

function closeMobileSidebar() {
  mobileOpen.value = false
  if (sidebarRef.value) {
    sidebarRef.value.classList.remove('overlay-open')
  }
}

function convInitial(c: Conversation): string {
  return (convName(c)[0] ?? '?').toUpperCase()
}

function convName(c: Conversation): string {
  return c.name ?? (c.conversation_type === 'direct'
    ? t('chat.common.directMessage')
    : t('chat.common.group'))
}

async function handleLogout() {
  const { logout } = useAuth()
  await logout()
}

const ws = useWebSocket()
onMounted(() => ws.connect())
onUnmounted(() => ws.disconnect())
</script>
