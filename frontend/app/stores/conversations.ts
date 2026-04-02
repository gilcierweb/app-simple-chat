import { defineStore } from 'pinia'
import type { Conversation, Message } from '~/types'

export const useConversationStore = defineStore('conversations', () => {
  const conversations = ref<Conversation[]>([])
  const activeConversationId = ref<string | null>(null)
  const messages = ref<Record<string, Message[]>>({})
  const typingUsers = ref<Record<string, Set<string>>>({})
  const loadingMessages = ref(false)
  const showNewChatModal = ref(false)

  const activeConversation = computed(() =>
    conversations.value.find(c => c.id === activeConversationId.value) ?? null,
  )

  const activeMessages = computed(() =>
    activeConversationId.value ? (messages.value[activeConversationId.value] ?? []) : [],
  )

  function setConversations(list: Conversation[]) {
    conversations.value = list
  }

  function upsertConversation(conv: Conversation) {
    const idx = conversations.value.findIndex(c => c.id === conv.id)
    if (idx >= 0) conversations.value[idx] = conv
    else conversations.value.unshift(conv)
  }

  function setActiveConversation(id: string | null) {
    activeConversationId.value = id
  }

  function setMessages(conversationId: string, msgs: Message[]) {
    messages.value[conversationId] = msgs
  }

  function isPlaceholder(msg?: Message | null): boolean {
    if (!msg) return false
    if (msg.is_placeholder) return true
    return msg.plaintext === '🔒 Encrypted message' || msg.plaintext === '[Unable to decrypt]'
  }

  function mergeMessage(existing: Message, incoming: Message): Message {
    // Preserve known-good plaintext when incoming payload is placeholder.
    if (existing.plaintext && !isPlaceholder(existing) && isPlaceholder(incoming)) {
      return {
        ...incoming,
        plaintext: existing.plaintext,
        is_placeholder: false,
      }
    }

    // If incoming has legitimate plaintext, it should win.
    if (incoming.plaintext && !isPlaceholder(incoming)) {
      return {
        ...existing,
        ...incoming,
        is_placeholder: false,
      }
    }

    // Otherwise keep the most complete version.
    return {
      ...existing,
      ...incoming,
    }
  }

  function appendMessage(msg: Message, replaceId?: string) {
    // Ensure the conversation key exists in the record
    if (!messages.value[msg.conversation_id]) {
      messages.value[msg.conversation_id] = []
    }

    const current = messages.value[msg.conversation_id] || []
    // Try to find by ID or the ID we want to replace
    const idx = current.findIndex(m => m.id === (replaceId || msg.id))

    if (idx >= 0) {
      // Create new array for reactivity
      const next = [...current]
      next[idx] = mergeMessage(next[idx], msg)
      messages.value[msg.conversation_id] = next
    } else {
      messages.value[msg.conversation_id] = [...current, msg]
    }

    // Update last_message on conversation
    const conv = conversations.value.find(c => c.id === msg.conversation_id)
    if (conv) {
      conv.last_message = msg
      conv.updated_at = msg.created_at
      // Re-sort by last activity
      conversations.value.sort((a, b) =>
        new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime(),
      )
    }
  }

  function deleteMessage(conversationId: string, messageId: string) {
    const msgs = messages.value[conversationId]
    if (msgs) {
      const idx = msgs.findIndex(m => m.id === messageId)
      if (idx >= 0) msgs.splice(idx, 1)
    }
  }

  function setTyping(conversationId: string, userId: string, isTyping: boolean) {
    if (!typingUsers.value[conversationId]) {
      typingUsers.value[conversationId] = new Set()
    }
    if (isTyping) typingUsers.value[conversationId].add(userId)
    else typingUsers.value[conversationId].delete(userId)
  }

  function getTypingUsers(conversationId: string): string[] {
    return [...(typingUsers.value[conversationId] ?? [])]
  }

  function openNewChatModal() {
    showNewChatModal.value = true
  }

  function closeNewChatModal() {
    showNewChatModal.value = false
  }

  return {
    conversations,
    activeConversationId,
    messages,
    loadingMessages,
    showNewChatModal,
    activeConversation,
    activeMessages,
    setConversations,
    upsertConversation,
    setActiveConversation,
    setMessages,
    appendMessage,
    deleteMessage,
    setTyping,
    getTypingUsers,
    openNewChatModal,
    closeNewChatModal,
  }
})
