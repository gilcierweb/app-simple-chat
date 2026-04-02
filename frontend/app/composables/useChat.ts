/**
 * useChat — High-level E2E chat orchestrator.
 *
 * Responsibilities:
 *  1. Load conversation list from server, hydrate the Pinia store.
 *  2. Open a conversation: fetch messages, decrypt each one, persist to IndexedDB.
 *  3. Send a message: encrypt → POST to Rust backend → broadcast via WS optimistically.
 *  4. Receive incoming WS messages: decrypt → store → update UI.
 *  5. Create new conversation (direct or group), triggering session key establishment.
 */

import type { Conversation, Message } from '~/types'

interface CreateConversationOpts {
  /** Target user's UUID */
  participantUserId?: string
  /** Target user's email (alternative to UUID) */
  participantEmail?: string
}

interface SendMessageOpts {
  conversationId: string
  plaintext: string
  /** The peer's user_id — needed to derive the session key  */
  peerUserId: string
}

export const useChat = () => {
  const config = useRuntimeConfig()
  const authStore = useAuthStore()
  const convStore = useConversationStore()
  const keyStore = useKeyStore()
  const messages = useMessages()
  const ws = useWebSocket()

  // ---------------------------------------------------------------------------
  // Helpers
  // ---------------------------------------------------------------------------

  function authHeaders() {
    return {
      Authorization: `Bearer ${authStore.accessToken}`,
      'Content-Type': 'application/json',
    }
  }

  // ---------------------------------------------------------------------------
  // 1. Load conversation list
  // ---------------------------------------------------------------------------

  async function loadConversations(): Promise<Conversation[]> {
    const list = await $fetch<Conversation[]>(`${config.public.apiBase}/conversations`, {
      headers: authHeaders(),
    })
    convStore.setConversations(list)
    return list
  }

  // ---------------------------------------------------------------------------
  // 2. Open conversation — fetch & decrypt messages
  // ---------------------------------------------------------------------------

  /**
   * Find the peer user ID in a direct conversation from the member list.
   */
  function getPeerUserId(conv: Conversation): string | null {
    const myId = authStore.user?.id
    const peer = conv.members?.find((m: { user_id: string }) => m.user_id !== myId)
    return peer?.user_id ?? null
  }

  async function openConversation(convId: string): Promise<void> {
    convStore.setActiveConversation(convId)
    const conv = convStore.conversations.find((c: Conversation) => c.id === convId)
    if (!conv) return

    const peerUserId = getPeerUserId(conv)

    // 1. Try loading from IndexedDB cache first (instant)
    const cached = await messages.loadLocal(convId)
    if (cached.length > 0) {
      convStore.setMessages(convId, cached)
    }

    // 2. Fetch latest from server
    convStore.loadingMessages = true
    try {
      const serverMsgs = await $fetch<Message[]>(
        `${config.public.apiBase}/conversations/${convId}/messages`,
        { headers: authHeaders() },
      )

      // Ensure session exists (will auto-fetch peer bundle if needed)
      if (peerUserId) {
        await keyStore.ensureSession(convId, peerUserId)
      }

      // Decrypt each message
      const decrypted = await decryptMessageBatch(serverMsgs, convId, peerUserId)
      convStore.setMessages(convId, decrypted)

      // Persist newly decrypted messages
      for (const msg of decrypted) {
        if (msg.plaintext && msg.plaintext !== '[Unable to decrypt]') {
          await messages.saveLocal(msg)
        }
      }
    }
    finally {
      convStore.loadingMessages = false
    }

    // 3. Join WebSocket room
    ws.joinRoom(convId)
  }

  async function decryptMessageBatch(
    msgs: Message[],
    convId: string,
    peerUserId: string | null,
  ): Promise<Message[]> {
    return Promise.all(msgs.map(msg => decryptMessage(msg, convId, peerUserId)))
  }

  async function decryptMessage(
    msg: Message,
    convId: string,
    peerUserId: string | null,
    isRetry = false,
  ): Promise<Message> {
    // Sender's own messages — we try the same session key
    const effectivePeer = peerUserId ?? msg.sender_id

    try {
      const sessionKey = await keyStore.getSessionKey(convId, effectivePeer)
      const plaintext = await keyStore.decrypt(sessionKey, msg.ciphertext, msg.iv)
      return { ...msg, plaintext, is_placeholder: false }
    }
    catch (e) {
      if (!isRetry && effectivePeer) {
        console.warn(`[useChat] Decryption failed, retrying with fresh session for peer ${effectivePeer}...`)
        await keyStore.invalidateSession(convId, effectivePeer)
        return decryptMessage(msg, convId, peerUserId, true)
      }
      
      // Graceful degradation — never crash the UI
      console.error('[useChat] Final decryption failure:', e)
      return { ...msg, plaintext: '[Unable to decrypt]', is_placeholder: true }
    }
  }

  // ---------------------------------------------------------------------------
  // 3. Send message — encrypt → HTTP → WS optimistic
  // ---------------------------------------------------------------------------

  async function sendMessage({ conversationId, plaintext, peerUserId }: SendMessageOpts): Promise<void> {
    // Ensure session key exists (fetch peer bundle if needed)
    const sessionKey = await keyStore.getSessionKey(conversationId, peerUserId)
    const { ciphertext, iv } = await keyStore.encrypt(sessionKey, plaintext)

    // Optimistic local message so the UI responds immediately
    const tempId = `temp-${Date.now()}`
    const optimistic: Message = {
      id: tempId,
      conversation_id: conversationId,
      sender_id: authStore.user?.id ?? '',
      ciphertext,
      iv,
      message_type: 'text',
      reply_to_id: null,
      created_at: new Date().toISOString(),
      deleted_at: null,
      plaintext,
      status: 'sending',
    }
    convStore.appendMessage(optimistic)

    try {
      // POST ciphertext as bytes — the backend stores Bytea directly.
      // We send it as base64 string; the server parses it as Vec<u8> via Serde.
      const saved = await $fetch<Message>(
        `${config.public.apiBase}/conversations/${conversationId}/messages`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: {
            ciphertext: Array.from(atob(ciphertext), c => c.charCodeAt(0)),
            iv,
            message_type: 1, // 1 = text in our schema
          },
        },
      )

      // Replace optimistic with server-confirmed message (keeps plaintext)
      const confirmed: Message = { ...saved, plaintext, status: 'sent', is_placeholder: false }
      convStore.appendMessage(confirmed, tempId)
      await messages.saveLocal(confirmed)
    }
    catch (e) {
      // Mark optimistic message as errored
      convStore.appendMessage({ ...optimistic, status: 'error' })
      throw e
    }
  }

  // ---------------------------------------------------------------------------
  // 4. Handle incoming WS messages (decrypt & store)
  // ---------------------------------------------------------------------------

  function handleIncomingMessage(raw: {
    type: string
    conversation_id?: string
    message_id?: string
    sender_id?: string
    ciphertext?: string
    iv?: string
    message_type?: string
    reply_to_id?: string | null
    created_at?: string
  }): void {
    if (raw.type !== 'new_message') return

    const convId = raw.conversation_id!
    const myId = authStore.user?.id

    // Ignore self-sent messages via WS (they are already handled by the HTTP response confirmation)
    if (raw.sender_id === myId) {
      console.log('Ignoring self-sent message via WS')
      return
    }

    const conv = convStore.conversations.find((c: Conversation) => c.id === convId)
    const peerUserId = conv ? getPeerUserId(conv) : raw.sender_id ?? null

    // Build a fully-typed Message from the WS payload
    const wsMsg: Message = {
      id: raw.message_id!,
      conversation_id: convId,
      sender_id: raw.sender_id!,
      ciphertext: raw.ciphertext!,
      iv: raw.iv!,
      message_type: raw.message_type as Message['message_type'] ?? 'text',
      reply_to_id: raw.reply_to_id ?? null,
      created_at: raw.created_at ?? new Date().toISOString(),
      deleted_at: null,
      plaintext: '🔒 Encrypted message',
      is_placeholder: true,
      status: 'delivered',
    }

    // Append placeholder immediately so the message appears
    convStore.appendMessage(wsMsg)

    // Decrypt async and update
    decryptMessage(wsMsg, convId, peerUserId).then(async (decrypted) => {
      convStore.appendMessage(decrypted)
      if (decrypted.plaintext && !decrypted.is_placeholder) {
        await messages.saveLocal(decrypted)
      }
    }).catch(console.error)
  }

  // ---------------------------------------------------------------------------
  // 5. Create new conversation
  // ---------------------------------------------------------------------------

  async function createConversation({ participantUserId, participantEmail }: CreateConversationOpts): Promise<Conversation> {
    const conv = await $fetch<Conversation>(`${config.public.apiBase}/conversations`, {
      method: 'POST',
      headers: authHeaders(),
      body: {
        participant_user_id: participantUserId ?? null,
        participant_email: participantEmail ?? null,
        conversation_type: 1, // direct
      },
    })

    convStore.upsertConversation(conv)

    // Pre-warm session key: fetch peer bundle now so first message is instant
    const peerUserId = participantUserId
      ?? conv.members?.find((m: { user_id: string }) => m.user_id !== authStore.user?.id)?.user_id
    if (peerUserId) {
      // Best-effort — non-blocking, errors are silent
      keyStore.ensureSession(conv.id, peerUserId).catch(console.warn)
    }

    return conv
  }

  return {
    loadConversations,
    openConversation,
    sendMessage,
    handleIncomingMessage,
    createConversation,
    getPeerUserId,
    decryptMessage,
  }
}
