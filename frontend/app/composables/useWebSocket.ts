import type { WsIncomingMessage } from '~/types'

type WsEventHandler = (msg: WsIncomingMessage) => void

let socket: WebSocket | null = null
let reconnectTimer: ReturnType<typeof setTimeout> | null = null
let pingTimer: ReturnType<typeof setInterval> | null = null
const handlers = new Set<WsEventHandler>()

export const useWebSocket = () => {
  const config = useRuntimeConfig()
  const connected = useState('ws:connected', () => false)

  function connect() {
    const token = localStorage.getItem('access_token')
    if (!token) return
    if (socket?.readyState === WebSocket.OPEN) return

    const url = `${config.public.wsUrl}?token=${encodeURIComponent(token)}`
    socket = new WebSocket(url)

    socket.onopen = () => {
      connected.value = true
      if (reconnectTimer) { clearTimeout(reconnectTimer); reconnectTimer = null }
      // Heartbeat ping every 20s
      pingTimer = setInterval(() => {
        if (socket?.readyState === WebSocket.OPEN) {
          socket.send(JSON.stringify({ type: 'ping' }))
        }
      }, 20_000)
    }

    socket.onmessage = (event) => {
      try {
        const msg: WsIncomingMessage = JSON.parse(event.data)
        handlers.forEach(h => h(msg))
      } catch {}
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
    send({ type: 'typing', conversation_id: conversationId })
  }

  function sendMarkRead(conversationId: string, messageId: string) {
    send({ type: 'mark_read', conversation_id: conversationId, message_id: messageId })
  }

  return { connected, connect, disconnect, send, on, sendTyping, sendMarkRead }
}
