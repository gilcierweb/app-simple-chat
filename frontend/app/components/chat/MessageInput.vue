<template>
  <div class="border-t border-base-300 bg-base-100 px-4 py-3">
    <form @submit.prevent="sendMessage" class="flex items-end gap-2">
      <!-- Attachment button -->
      <button type="button" class="btn btn-ghost btn-sm btn-square text-base-content/50 flex-shrink-0 mb-0.5" title="Attach file">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
        </svg>
      </button>

      <!-- Text area -->
      <div class="flex-1 relative">
        <textarea
          ref="textareaEl"
          v-model="text"
          placeholder="Type a message..."
          class="textarea textarea-bordered w-full resize-none text-sm leading-relaxed min-h-[42px] max-h-[160px] pr-10"
          rows="1"
          :disabled="disabled || sending"
          @keydown.enter.exact.prevent="sendMessage"
          @keydown.enter.shift.exact="() => {}"
          @input="onInput"
        ></textarea>
        <!-- Emoji button -->
        <button type="button" class="absolute right-2 bottom-2 text-base-content/30 hover:text-base-content/70" title="Emoji">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>

      <!-- Send button -->
      <button
        type="submit"
        class="btn btn-primary btn-sm btn-square flex-shrink-0 mb-0.5"
        :disabled="!text.trim() || disabled || sending"
      >
        <span v-if="sending" class="loading loading-spinner loading-xs"></span>
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
        </svg>
      </button>
    </form>

    <!-- Encryption notice -->
    <p class="text-center text-xs text-base-content/30 mt-2 flex items-center justify-center gap-1">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
      </svg>
      End-to-end encrypted
    </p>
  </div>
</template>

<script setup lang="ts">
import type { Message } from '~/types'
import { useAuthStore } from '~/stores/auth'
import { useConversationStore } from '~/stores/conversations'

const props = defineProps<{
  conversationId: string
  disabled?: boolean
}>()

const emit = defineEmits<{
  sent: [msg: Message]
}>()

const { authFetch } = useAuth()
const keyStore = useKeyStore()
const ws = useWebSocket()

const text = ref('')
const sending = ref(false)
const textareaEl = ref<HTMLTextAreaElement>()

let typingTimeout: ReturnType<typeof setTimeout> | null = null

function onInput() {
  // Auto-resize textarea
  const el = textareaEl.value
  if (el) {
    el.style.height = 'auto'
    el.style.height = Math.min(el.scrollHeight, 160) + 'px'
  }

  // Send typing event (debounced)
  ws.sendTyping(props.conversationId)
  if (typingTimeout) clearTimeout(typingTimeout)
  typingTimeout = setTimeout(() => { typingTimeout = null }, 2000)
}

async function sendMessage() {
  const content = text.value.trim()
  console.log('sendMessage called, content:', content)
  if (!content || props.disabled || sending.value) return

  sending.value = true
  const optimisticId = crypto.randomUUID()

  try {
    // Get peer user ID from conversation members
    const convStore = useConversationStore()
    const conversation = convStore.conversations.find(c => c.id === props.conversationId)
    const currentUserId = useAuthStore().user?.id
    const peerUserId = conversation?.members?.find(m => m.user_id !== currentUserId)?.user_id

    if (!peerUserId) {
      console.error('Could not find peer user ID')
      throw new Error('Cannot find peer user')
    }

    // Encrypt message client-side
    console.log('Getting session key for:', props.conversationId, 'peer:', peerUserId)
    
    // Try to get session key for sending, will fetch bundle if not exists
    let sessionKey = await keyStore.getSessionKey(props.conversationId, peerUserId).catch(() => null)
    
    if (!sessionKey) {
      console.log('No session key - creating session')
      await keyStore.ensureSession(props.conversationId, peerUserId)
      sessionKey = await keyStore.getSessionKey(props.conversationId, peerUserId)
      if (!sessionKey) {
        throw new Error('Need peer bundle to establish new session')
      }
    }
    
    console.log('Session key obtained:', sessionKey)
    const { ciphertext, iv } = await keyStore.encrypt(sessionKey, content)
    console.log('Encrypted message, ciphertext length:', ciphertext.length)

    text.value = ''
    if (textareaEl.value) textareaEl.value.style.height = 'auto'

    // Send to server (no optimistic - will be added via WebSocket)
    console.log('Sending to server...')
    const msg = await authFetch<Message>(`/messages/${props.conversationId}`, {
      method: 'POST',
      body: JSON.stringify({ ciphertext, iv, message_type: 1 }),
    })
    console.log('Message sent:', msg)
    
    // Emit so the sender sees the message immediately
    emit('sent', { ...msg, plaintext: content, status: 'sent' })
  } catch (e) {
    console.error('Failed to send message', e)
    // Mark optimistic message as failed
  } finally {
    sending.value = false
  }
}
</script>
