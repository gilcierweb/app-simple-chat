import type { WsIncomingMessage } from '~/types'

type WsEventHandler = (msg: WsIncomingMessage) => void

let socket: WebSocket | null = null
let reconnectTimer: ReturnType<typeof setTimeout> | null = null
let pingTimer: ReturnType<typeof setInterval> | null = null
const handlers = new Set<WsEventHandler>()

export const useWebSocket = () => {
  const config = useRuntimeConfig()
  const connected = useState('ws:connected', () => false)
  const authStore = useAuthStore()

  function connect() {
    const token = authStore.accessToken
    if (!token) return
    if (socket?.readyState === WebSocket.OPEN) return

    const url = config.public.wsUrl as string
    socket = new WebSocket(url)

    socket.onopen = () => {
      // First-Message Protocol: Send token securely inside the tunnel
      socket?.send(JSON.stringify({ action: 'auth', data: { token } }))
      
      connected.value = true
      if (reconnectTimer) { clearTimeout(reconnectTimer); reconnectTimer = null }
      
      // Heartbeat ping every 20s
      pingTimer = setInterval(() => {
        if (socket?.readyState === WebSocket.OPEN) {
          socket.send(JSON.stringify({ action: 'ping', data: {} }))
        }
      }, 20_000)
    }

    socket.onmessage = (event) => {
      console.log('WS received:', event.data)
      try {
        const raw = JSON.parse(event.data)
        console.log('WS raw msg_type:', raw.msg_type)
        // Transform backend format to frontend format
        let msg: WsIncomingMessage
        if (raw.msg_type === 'new_message') {
          msg = {
            type: 'new_message',
            conversation_id: raw.payload.conversation_id,
            message_id: raw.payload.message_id,
            sender_id: raw.payload.sender_id,
            ciphertext: raw.payload.ciphertext,
            iv: raw.payload.iv,
            message_type: raw.payload.message_type,
            reply_to_id: raw.payload.reply_to_id,
            created_at: raw.payload.created_at,
          }
        } else if (raw.msg_type === 'typing') {
          msg = {
            type: 'typing',
            conversation_id: raw.payload.room,
            user_id: raw.payload.user_id || raw.payload.user,
          }
        } else {
          msg = raw
        }
        console.log('WS parsed:', msg)
        handlers.forEach(h => h(msg))
      } catch (e) {
        console.error('WS parse error:', e)
      }
    }

    socket.onclose = () => {
      connected.value = false
      if (pingTimer) { clearInterval(pingTimer); pingTimer = null }
      // Auto-reconnect with backoff
      reconnectTimer = setTimeout(connect, 3_000)
    }

    socket.onerror = () => {
      socket?.close()
    }
  }

  function disconnect() {
    if (reconnectTimer) clearTimeout(reconnectTimer)
    if (pingTimer) clearInterval(pingTimer)
    socket?.close()
    socket = null
    connected.value = false
  }

  function send(data: object) {
    if (socket?.readyState === WebSocket.OPEN) {
      socket.send(JSON.stringify(data))
    }
  }

  function on(handler: WsEventHandler) {
    handlers.add(handler)
    return () => handlers.delete(handler)
  }

  function sendTyping(conversationId: string) {
    send({ action: 'typing', data: { room: conversationId } })
  }

  function sendMarkRead(conversationId: string, messageId: string) {
    send({ action: 'mark_read', data: { conversation_id: conversationId, message_id: messageId } })
  }

  function joinRoom(conversationId: string) {
    send({ action: 'join_room', data: { room: conversationId } })
  }

  function leaveRoom(conversationId: string) {
    send({ action: 'leave_room', data: { room: conversationId } })
  }

  return { connected, connect, disconnect, send, on, sendTyping, sendMarkRead, joinRoom, leaveRoom }
}
