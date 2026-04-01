<template>
  <div class="border-t border-base-300 bg-base-100 px-4 py-3">
    <form @submit.prevent="sendMessage" class="flex items-end gap-2">
      <!-- Attachment button -->
      <button type="button" class="btn btn-ghost btn-sm btn-square text-base-content/50 flex-shrink-0" title="Attach file">
        <span class="icon-[tabler--paperclip] size-5"></span>
      </button>

      <!-- Text area with FlyonUI input style -->
      <div class="flex-1 relative input input-bordered p-0 min-h-[48px]">
        <textarea
          ref="textareaEl"
          v-model="text"
          placeholder="Type a message..."
          class="grow bg-transparent border-none focus:ring-0 resize-none text-sm leading-relaxed min-h-[42px] max-h-[160px] px-3 py-2.5"
          rows="1"
          :disabled="disabled || sending"
          @keydown.enter.exact.prevent="sendMessage"
          @keydown.enter.shift.exact="() => {}"
          @input="onInput"
        ></textarea>
        <!-- Emoji button -->
        <button type="button" class="absolute right-2 bottom-2 text-base-content/40 hover:text-base-content/70" title="Emoji">
          <span class="icon-[tabler--mood-smile] size-5"></span>
        </button>
      </div>

      <!-- Send button -->
      <button
        type="submit"
        class="btn btn-primary btn-sm btn-square flex-shrink-0"
        :disabled="!text.trim() || disabled || sending"
      >
        <span v-if="sending" class="loading loading-spinner loading-xs"></span>
        <span v-else class="icon-[tabler--send] size-4"></span>
      </button>
    </form>

    <!-- Encryption notice -->
    <p class="text-center text-xs text-base-content/40 mt-2 flex items-center justify-center gap-1">
      <span class="icon-[tabler--lock] size-3"></span>
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
