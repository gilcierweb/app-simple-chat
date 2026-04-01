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

  function appendMessage(msg: Message) {
    // Create new array to ensure reactivity
    const current = messages.value[msg.conversation_id] || []
    // Avoid duplicates
    if (!current.some(m => m.id === msg.id)) {
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
