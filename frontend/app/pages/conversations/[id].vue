<template>
  <div class="flex flex-col h-full min-h-0">
    <!-- Chat header -->
    <header class="flex items-center gap-3 px-4 py-3 border-b border-dark-800 bg-dark-900">
      <div class="avatar">
        <div class="w-9 rounded-full bg-brand-600 text-white flex items-center justify-center font-bold text-sm">
          {{ conversationInitial }}
        </div>
      </div>
      <div class="flex-1 min-w-0">
        <p class="font-semibold text-sm text-text-primary truncate">{{ conversationName }}</p>
        <p v-if="typingText" class="text-xs text-success">{{ typingText }}</p>
        <p v-else class="text-xs text-text-muted">
          {{ conversation?.conversation_type === 'group' ? memberCount + ' members' : 'Direct message' }}
        </p>
      </div>
      <div class="flex items-center gap-1">
        <button class="btn btn-ghost btn-sm btn-square text-text-muted" title="Search messages" @click="showSearch = !showSearch">
          <span class="icon-[lucide--search] size-5"></span>
        </button>
        <button class="btn btn-ghost btn-sm btn-square text-text-muted" title="Conversation info">
          <span class="icon-[lucide--info] size-5"></span>
        </button>
      </div>
    </header>

    <!-- Search bar -->
    <div v-if="showSearch" class="px-4 py-2 bg-dark-900 border-b border-dark-800">
      <div class="input input-bordered input-sm border-dark-700 bg-dark-950 flex items-center gap-2">
        <span class="icon-[lucide--search] size-4 text-text-muted"></span>
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search in this conversation..."
          class="bg-transparent border-none outline-none w-full text-text-primary placeholder:text-text-muted"
          @input="onSearch"
        />
      </div>
    </div>

    <!-- Messages area -->
    <div ref="messagesEl" class="flex-1 overflow-y-auto min-h-0 px-4 py-4">
      <div v-if="loading" class="flex justify-center py-8">
        <span class="loading loading-spinner loading-md text-primary"></span>
      </div>

      <TransitionGroup name="message" tag="div" class="space-y-1">
        <template v-for="(msg, idx) in displayMessages" :key="msg.id">
          <!-- Date separator -->
          <div v-if="showDateSeparator(msg, displayMessages[idx - 1])" class="flex items-center gap-3 my-4">
            <div class="flex-1 h-px bg-dark-700"></div>
            <span class="text-xs text-text-muted">{{ formatDate(msg.created_at) }}</span>
            <div class="flex-1 h-px bg-dark-700"></div>
          </div>

          <MessageBubble
            :message="msg"
            :is-own="msg.sender_id === authStore.user?.id"
            :show-avatar="shouldShowAvatar(msg, displayMessages[idx - 1])"
            @delete="deleteMessage(msg.id)"
          />
        </template>
      </TransitionGroup>

      <!-- Typing indicator with reserved space -->
      <div class="h-10 transition-all duration-300 ease-out" :class="{ 'opacity-0': typingUserIds.length === 0, 'opacity-100': typingUserIds.length > 0 }">
        <div class="flex items-end gap-2">
          <div class="w-8 h-8 rounded-full bg-dark-700 flex-shrink-0"></div>
          <div class="px-3 py-2 bg-dark-800 rounded-lg flex items-center gap-1">
            <span class="typing-dot"></span>
            <span class="typing-dot"></span>
            <span class="typing-dot"></span>
          </div>
        </div>
      </div>

      <div ref="bottomEl"></div>
    </div>

    <!-- Message input -->
    <MessageInput
      :conversation-id="conversationId"
      :disabled="false"
      @sent="onMessageSent"
    />
  </div>
</template>

<script setup lang="ts">
import type { Message } from '~/types'
import { useAuthStore } from '~/stores/auth'
import { useConversationStore } from '~/stores/conversations'
import MessageBubble from '~/components/chat/MessageBubble.vue'
import MessageInput from '~/components/chat/MessageInput.vue'

definePageMeta({
  layout: 'default',
  requiresAuth: true,
})

const route = useRoute()
const conversationId = route.params.id as string

const authStore = useAuthStore()
const convStore = useConversationStore()
const messageStore = useMessages()
const ws = useWebSocket()
const chat = useChat()

const loading = computed(() => convStore.loadingMessages)
const showSearch = ref(false)
const searchQuery = ref('')
const searchResults = ref<Message[]>([])
const messagesEl = ref<HTMLElement>()
const bottomEl = ref<HTMLElement>()

const conversation = computed(() =>
  convStore.conversations.find((c: { id: string }) => c.id === conversationId)
)
const conversationName = computed(() => conversation.value?.name ?? 'Direct message')
const conversationInitial = computed(() => (conversationName.value[0] ?? '?').toUpperCase())
const memberCount = computed(() => conversation.value?.members?.length ?? 0)
const typingUserIds = computed(() => convStore.getTypingUsers(conversationId))

const typingText = computed(() => {
  const ids = typingUserIds.value
  if (ids.length === 0) return ''
  if (ids.length === 1) return 'typing...'
  return `${ids.length} people typing...`
})

const displayMessages = computed(() =>
  showSearch.value && searchQuery.value ? searchResults.value : convStore.activeMessages,
)

function showDateSeparator(msg: Message, prev?: Message): boolean {
  if (!prev) return true
  return new Date(prev.created_at).toDateString() !== new Date(msg.created_at).toDateString()
}

function shouldShowAvatar(msg: Message, prev?: Message): boolean {
  if (!prev) return true
  return prev.sender_id !== msg.sender_id
}

function formatDate(iso: string): string {
  const d = new Date(iso)
  const today = new Date()
  if (d.toDateString() === today.toDateString()) return 'Today'
  const yesterday = new Date(today)
  yesterday.setDate(today.getDate() - 1)
  if (d.toDateString() === yesterday.toDateString()) return 'Yesterday'
  return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })
}

function scrollToBottom(smooth = false) {
  nextTick(() => {
    if (!messagesEl.value || !bottomEl.value) return
    const container = messagesEl.value
    const target = bottomEl.value
    const targetPosition = target.offsetTop - container.offsetTop + target.offsetHeight
    smooth
      ? container.scrollTo({ top: targetPosition, behavior: 'smooth' })
      : (container.scrollTop = targetPosition)
  })
}

async function onSearch() {
  if (!searchQuery.value.trim()) { searchResults.value = []; return }
  searchResults.value = await messageStore.searchLocal(searchQuery.value)
}

function onMessageSent(msg: Message) {
  convStore.appendMessage(msg)
  if (msg.plaintext && !msg.is_placeholder && msg.sender_id !== 'system') {
    void messageStore.saveLocal(msg)
  }
  scrollToBottom(true)
}

const { authFetch } = useAuth()

async function deleteMessage(messageId: string) {
  try {
    await authFetch(`/conversations/${conversationId}/messages/${messageId}`, { method: 'DELETE' })
    convStore.deleteMessage(conversationId, messageId)
  }
  catch (e) {
    console.error('Failed to delete message', e)
  }
}

let unsubscribeWs: (() => void) | null = null

onMounted(async () => {
  // Load conversation list if store empty
  if (convStore.conversations.length === 0) {
    await chat.loadConversations().catch(console.error)
  }

  // openConversation: IndexedDB cache → server fetch → batch decrypt → join WS room
  await chat.openConversation(conversationId)
  scrollToBottom()

  // Real-time WS subscription
  unsubscribeWs = ws.on(async (event) => {
    if (event.type === 'new_message' && event.conversation_id === conversationId) {
      chat.handleIncomingMessage(event as any)
      scrollToBottom(true)
      ws.sendMarkRead(conversationId, event.message_id)
    }
    if (event.type === 'typing' && event.conversation_id === conversationId) {
      convStore.setTyping(conversationId, event.user_id, true)
      setTimeout(() => convStore.setTyping(conversationId, event.user_id, false), 3000)
    }
  })
})

onUnmounted(() => {
  ws.leaveRoom(conversationId)
  if (convStore.activeConversationId === conversationId) {
    convStore.setActiveConversation(null)
  }
  unsubscribeWs?.()
})
</script>

<style scoped>
/* Message enter animation */
.message-enter-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.message-leave-active {
  transition: all 0.2s ease-out;
}

.message-enter-from,
.message-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

/* Typing dots animation */
.typing-dot {
  width: 6px;
  height: 6px;
  background: currentColor;
  border-radius: 50%;
  animation: typing-bounce 1.4s infinite ease-in-out both;
}

.typing-dot:nth-child(1) { animation-delay: -0.32s; }
.typing-dot:nth-child(2) { animation-delay: -0.16s; }

@keyframes typing-bounce {
  0%, 80%, 100% { transform: scale(0.6); opacity: 0.4; }
  40% { transform: scale(1); opacity: 1; }
}
</style>
