<template>
  <div class="border-t border-base-300 bg-base-100 px-4 py-3 shrink-0">
    <form @submit.prevent="sendMessage" class="flex items-end gap-2 max-h-[120px]">
      <!-- Attachment button -->
      <button type="button" class="btn btn-ghost btn-sm btn-square text-base-content/50 flex-shrink-0 mb-1" :title="t('chat.input.attachFile')">
        <span class="icon-[tabler--paperclip] size-5"></span>
      </button>

      <!-- Text area with FlyonUI input style -->
      <div class="flex-1 relative input input-bordered p-0 h-auto min-h-[48px] max-h-[96px] overflow-hidden">
        <textarea
          ref="textareaEl"
          v-model="text"
          :placeholder="t('chat.input.placeholder')"
          class="grow bg-transparent border-none focus:ring-0 resize-none text-sm leading-relaxed min-h-[42px] max-h-[96px] px-3 py-2.5 w-full"
          rows="1"
          :disabled="disabled"
          @keydown.enter.exact.prevent="sendMessage"
          @keydown.enter.shift.exact="() => {}"
          @input="onInput"
        ></textarea>
        <!-- Emoji button -->
        <button type="button" class="absolute right-2 bottom-2 text-base-content/40 hover:text-base-content/70" :title="t('chat.input.emoji')">
          <span class="icon-[tabler--mood-smile] size-5"></span>
        </button>
      </div>

      <!-- Send button -->
      <button
        type="submit"
        class="btn btn-primary btn-sm btn-square flex-shrink-0 mb-1"
        :disabled="!text.trim() || disabled || sending"
      >
        <span v-if="sending" class="loading loading-spinner loading-xs"></span>
        <span v-else class="icon-[tabler--send] size-4"></span>
      </button>
    </form>

    <!-- Encryption notice -->
    <p class="text-center text-xs text-base-content/40 mt-2 flex items-center justify-center gap-1">
      <span class="icon-[tabler--lock] size-3"></span>
      {{ t('chat.input.e2eNotice') }}
    </p>
  </div>
</template>

<script setup lang="ts">
import type { Message, Conversation } from '~/types'
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
const { t } = useI18n()
const toast = useToast()

const text = ref('')
const sending = ref(false)
const textareaEl = ref<HTMLTextAreaElement>()

let typingTimeout: ReturnType<typeof setTimeout> | null = null

function onInput() {
  // Auto-resize textarea with strict limits
  const el = textareaEl.value
  if (el) {
    el.style.height = 'auto'
    const newHeight = Math.min(el.scrollHeight, 96)
    el.style.height = newHeight + 'px'
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
    console.log('[DEBUG] All conversations:', convStore.conversations)
    console.log('[DEBUG] Looking for conversation ID:', props.conversationId)
    const conversation = convStore.conversations.find((c: Conversation) => c.id === props.conversationId)
    console.log('[DEBUG] Found conversation:', conversation)
    console.log('[DEBUG] Conversation members:', conversation?.members)
    const currentUserId = useAuthStore().user?.id
    console.log('[DEBUG] Current user ID:', currentUserId)
    // @ts-ignore - member structure is not fully typed
    const peerUserId = conversation?.members?.find((m) => m.user_id !== currentUserId)?.user_id
    console.log('[DEBUG] Peer user ID found:', peerUserId)

    if (!peerUserId) {
      console.error('Could not find peer user ID')
      throw new Error(t('chat.input.errors.cannotFindPeer'))
    }

    // Encrypt message client-side
    console.log('Getting session key for:', props.conversationId, 'peer:', peerUserId)
    
    // Always fetch latest peer bundle before encrypting.
    // This avoids using stale cached session keys after peer key rotation.
    const bundle = await keyStore.fetchPeerBundle(peerUserId)
    if (!bundle) {
      throw new Error(t('chat.input.errors.peerNoKeys'))
    }
    const sessionKey = await keyStore.getSessionKey(props.conversationId, peerUserId, bundle).catch(() => null)
    if (!sessionKey) {
      throw new Error(t('chat.input.errors.needPeerBundle'))
    }
    
    console.log('Session key obtained:', sessionKey)
    const { ciphertext, iv } = await keyStore.encrypt(sessionKey, content)
    console.log('Encrypted message, ciphertext length:', ciphertext.length)

    text.value = ''
    if (textareaEl.value) {
      textareaEl.value.style.height = 'auto'
    }

    // Send to server (no optimistic - will be added via WebSocket)
    console.log('Sending to server...')
    const msg = await authFetch<Message>(`/messages/${props.conversationId}`, {
      method: 'POST',
      body: JSON.stringify({ ciphertext, iv, message_type: 1 }),
    })
    console.log('Message sent:', msg)
    
    // Emit so the sender sees the message immediately
    emit('sent', { ...msg, plaintext: content, status: 'sent' })
  } catch (e: any) {
    console.error('Failed to send message', e)
    // Show user-friendly error for missing encryption keys
    if (e?.message?.includes('encryption keys')) {
      toast.error(e.message, { duration: 7000 })
    }
    // Mark optimistic message as failed
  } finally {
    sending.value = false
    // Restore focus after textarea is re-enabled - use requestAnimationFrame for reliability
    requestAnimationFrame(() => {
      setTimeout(() => {
        textareaEl.value?.focus()
      }, 10)
    })
  }
}
</script>
