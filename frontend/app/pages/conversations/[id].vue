<template>
  <div class="flex flex-col h-dvh">
    <!-- Chat header -->
    <header class="flex items-center gap-3 px-4 py-3 border-b border-base-300 bg-base-100">
      <div class="avatar">
        <div class="w-9 rounded-full bg-secondary text-secondary-content flex items-center justify-center font-bold text-sm">
          {{ conversationInitial }}
        </div>
      </div>
      <div class="flex-1 min-w-0">
        <p class="font-semibold text-sm truncate">{{ conversationName }}</p>
        <p v-if="typingText" class="text-xs text-success">{{ typingText }}</p>
        <p v-else class="text-xs text-base-content/50">
          {{ conversation?.conversation_type === 'group' ? memberCount + ' members' : 'Direct message' }}
        </p>
      </div>
      <div class="flex items-center gap-1">
        <button class="btn btn-ghost btn-sm btn-square" title="Search messages" @click="showSearch = !showSearch">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </button>
        <button class="btn btn-ghost btn-sm btn-square" title="Conversation info">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </header>

    <!-- Search bar -->
    <div v-if="showSearch" class="px-4 py-2 bg-base-100 border-b border-base-300">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search in this conversation..."
        class="input input-sm w-full"
        @input="onSearch"
      />
    </div>

    <!-- Messages area -->
    <div ref="messagesEl" class="flex-1 overflow-y-auto px-4 py-4 space-y-1">
      <div v-if="loading" class="flex justify-center py-8">
        <span class="loading loading-spinner loading-md text-primary"></span>
      </div>

      <template v-for="(msg, idx) in displayMessages" :key="msg.id">
        <!-- Date separator -->
        <div v-if="showDateSeparator(msg, displayMessages[idx - 1])" class="flex items-center gap-3 my-4">
          <div class="flex-1 h-px bg-base-300"></div>
          <span class="text-xs text-base-content/40">{{ formatDate(msg.created_at) }}</span>
          <div class="flex-1 h-px bg-base-300"></div>
        </div>

        <MessageBubble
          :message="msg"
          :is-own="msg.sender_id === authStore.user?.id"
          :show-avatar="shouldShowAvatar(msg, displayMessages[idx - 1])"
          @delete="deleteMessage(msg.id)"
        />
      </template>

      <!-- Typing indicator -->
      <div v-if="typingUserIds.length > 0" class="flex items-end gap-2 mt-1">
        <div class="w-8 h-8 rounded-full bg-base-300 flex-shrink-0"></div>
        <div class="msg-in px-3 py-2 flex items-center gap-1">
          <span class="typing-dot"></span>
          <span class="typing-dot"></span>
          <span class="typing-dot"></span>
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

definePageMeta({ layout: 'default' })

const route = useRoute()
const conversationId = route.params.id as string

const authStore = useAuthStore()
console.log('[DEBUG] authStore.user:', authStore.user, 'authStore.user?.id:', authStore.user?.id)
const convStore = useConversationStore()
const { authFetch } = useAuth()
const keyStore = useKeyStore()
const messageStore = useMessages()
const ws = useWebSocket()

const loading = ref(false)
const showSearch = ref(false)
const searchQuery = ref('')
const searchResults = ref<Message[]>([])
const messagesEl = ref<HTMLElement>()
const bottomEl = ref<HTMLElement>()

const conversation = computed(() => convStore.conversations.find(c => c.id === conversationId))
const conversationName = computed(() => conversation.value?.name ?? 'Unknown')
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
  const a = new Date(prev.created_at).toDateString()
  const b = new Date(msg.created_at).toDateString()
  return a !== b
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

async function loadMessages() {
  loading.value = true
  convStore.setActiveConversation(conversationId)
  try {
    // First load from local IndexedDB
    const local = await messageStore.loadLocal(conversationId)
    convStore.setMessages(conversationId, local)

    // Then fetch from server and decrypt
    const remote: Message[] = await authFetch(`/messages/${conversationId}`)
    const peerUserId = conversation.value?.members?.find(m => m.user_id !== authStore.user?.id)?.user_id
    const sessionKey = peerUserId ? await keyStore.getSessionKey(conversationId, peerUserId).catch(() => null) : null

    const decrypted = await Promise.all(
      remote.map(async (msg) => {
        if (!sessionKey || msg.deleted_at) return msg
        try {
          const plaintext = await keyStore.decrypt(sessionKey, msg.ciphertext, msg.iv)
          return { ...msg, plaintext }
        } catch {
          return { ...msg, plaintext: '[Unable to decrypt]' }
        }
      }),
    )

    convStore.setMessages(conversationId, decrypted.reverse())
    // Persist locally
    await Promise.all(decrypted.map(m => messageStore.saveLocal(m)))
  } catch (e) {
    console.error('Failed to load messages', e)
  } finally {
    loading.value = false
    scrollToBottom()
  }
}

function scrollToBottom(smooth = false) {
  nextTick(() => {
    bottomEl.value?.scrollIntoView({ behavior: smooth ? 'smooth' : 'instant' })
  })
}

async function onSearch() {
  if (!searchQuery.value.trim()) { searchResults.value = []; return }
  searchResults.value = await messageStore.searchLocal(searchQuery.value)
}

function onMessageSent(msg: Message) {
  convStore.appendMessage(msg)
  scrollToBottom(true)
}

async function deleteMessage(messageId: string) {
  try {
    await authFetch(`/messages/${conversationId}/${messageId}`, { method: 'DELETE' })
    convStore.deleteMessage(conversationId, messageId)
  } catch (e) {
    console.error('Failed to delete message', e)
  }
}

// Handle incoming WS messages
let unsubscribeWs: (() => void) | null = null

onMounted(async () => {
  await loadMessages()
  
  // Join the conversation room for real-time updates
  ws.joinRoom(conversationId)

  unsubscribeWs = ws.on(async (event) => {
    console.log('WS handler received:', event, 'type:', event?.type, 'conversationId:', conversationId)
    if (event.type === 'new_message' && event.conversation_id === conversationId) {
      console.log('Processing new_message for conversation:', conversationId, 'event.conv:', event.conversation_id, 'MATCH!')
      // Use sender_id to get the correct session key for decryption
      const senderId = event.sender_id
      console.log('Sender ID for decryption:', senderId)
      // Fetch peer bundle from sender and establish/get session key
      let sessionKey: CryptoKey | null = null
      try {
        const bundle = await keyStore.fetchPeerBundle(senderId)
        if (bundle) {
          sessionKey = await keyStore.getSessionKey(conversationId, senderId, bundle)
        }
      } catch (err) {
        console.error('Failed to get session key:', err)
      }
      console.log('Session key:', sessionKey)
      let plaintext = '[Unable to decrypt]'
      if (sessionKey) {
        try { 
          plaintext = await keyStore.decrypt(sessionKey, event.ciphertext, event.iv) 
        } catch (err) { 
          console.error('Decrypt error:', err) 
        }
      }
      const msg: Message = {
        id: event.message_id,
        conversation_id: event.conversation_id,
        sender_id: event.sender_id,
        ciphertext: event.ciphertext,
        iv: event.iv,
        message_type: event.message_type as any,
        reply_to_id: event.reply_to_id,
        created_at: event.created_at,
        deleted_at: null,
        plaintext,
        status: 'delivered',
      }
      console.log('Adding message to store:', msg)
      convStore.appendMessage(msg)
      await messageStore.saveLocal(msg)
      scrollToBottom(true)

      // Mark as read if this conversation is active
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
  convStore.setActiveConversation(null)
  unsubscribeWs?.()
})
</script>
