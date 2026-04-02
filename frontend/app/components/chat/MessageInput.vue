<template>
  <div class="border-t border-dark-800 bg-dark-900 px-4 py-3 shrink-0">
    <form @submit.prevent="sendMessage" class="flex items-end gap-2 max-h-[120px]">
      <!-- Attachment button -->
      <button type="button" class="btn btn-ghost btn-sm btn-square text-text-muted flex-shrink-0 mb-1" :title="t('chat.input.attachFile')">
        <span class="icon-[lucide--paperclip] size-5"></span>
      </button>

      <!-- Text area with FlyonUI input style -->
      <div class="flex-1 relative input input-bordered border-dark-700 bg-dark-950 p-0 h-auto min-h-[48px] max-h-[96px] overflow-hidden">
        <textarea
          ref="textareaEl"
          v-model="text"
          :placeholder="t('chat.input.placeholder')"
          class="grow bg-transparent border-none focus:ring-0 resize-none text-sm leading-relaxed min-h-[42px] max-h-[96px] px-4 py-3 w-full text-text-primary placeholder:text-text-muted"
          rows="1"
          :disabled="disabled"
          @keydown.enter.exact.prevent="sendMessage"
          @keydown.enter.shift.exact="() => {}"
          @input="onInput"
        ></textarea>
        <!-- Emoji button -->
        <button type="button" class="absolute right-3 bottom-3 text-text-muted hover:text-text-secondary transition-colors" :title="t('chat.input.emoji')">
          <span class="icon-[lucide--smile] size-5"></span>
        </button>
      </div>

      <!-- Send button -->
      <button
        type="submit"
        class="btn btn-primary btn-sm btn-square flex-shrink-0 mb-1"
        :disabled="!text.trim() || disabled || sending"
      >
        <span v-if="sending" class="loading loading-spinner loading-xs"></span>
        <span v-else class="icon-[lucide--send] size-4"></span>
      </button>
    </form>

    <!-- Encryption notice -->
    <p class="text-center text-xs text-text-muted mt-2 flex items-center justify-center gap-1">
      <span class="icon-[lucide--lock] size-3"></span>
      {{ t('chat.input.e2eNotice') }}
    </p>
  </div>
</template>

<script setup lang="ts">
import type { Message, Conversation } from '~/types'

const props = defineProps<{
  conversationId: string
  disabled?: boolean
}>()

const emit = defineEmits<{
  sent: [msg: Message]
}>()

const keyStore = useKeyStore()
const ws = useWebSocket()
const { t } = useI18n()
const toast = useToast()
const convStore = useConversationStore()
const authStore = useAuthStore()

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

const { sendMessage: dispatchSend } = useChat()

async function sendMessage() {
  const content = text.value.trim()
  if (!content || props.disabled || sending.value) return

  sending.value = true

  try {
    // Resolve peer user ID from store
    const conversation = convStore.conversations.find(
      (c: Conversation) => c.id === props.conversationId
    )
    const currentUserId = authStore.user?.id
    const peerUserId = conversation?.members?.find(
      (m: { user_id: string }) => m.user_id !== currentUserId
    )?.user_id

    if (!peerUserId) {
      throw new Error(t('chat.input.errors.cannotFindPeer'))
    }

    // Use the central useChat orchestrator
    // It handles: Session key derivation, encryption, HTTP POST, and local state update
    await dispatchSend({
      conversationId: props.conversationId,
      plaintext: content,
      peerUserId
    })

    // Clear textarea upon success
    text.value = ''
    if (textareaEl.value) textareaEl.value.style.height = 'auto'
  }
  catch (e: any) {
    console.error('Failed to send message', e)
    const userMsg = e?.message ?? t('chat.input.errors.sendFailed', 'Failed to send')
    toast.error(userMsg, { duration: 6000 })
  }
  finally {
    sending.value = false
    requestAnimationFrame(() => {
      setTimeout(() => textareaEl.value?.focus(), 10)
    })
  }
}
</script>
