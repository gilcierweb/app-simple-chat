import { openDB } from 'idb'
import type { Message } from '~/types'

const DB_NAME = 'simple-chat-messages'
const DB_VERSION = 1

async function getDb() {
  return openDB(DB_NAME, DB_VERSION, {
    upgrade(db) {
      if (!db.objectStoreNames.contains('messages')) {
        const store = db.createObjectStore('messages', { keyPath: 'id' })
        store.createIndex('by_conversation', 'conversation_id')
        store.createIndex('by_conv_created', ['conversation_id', 'created_at'])
      }
    },
  })
}

export const useMessages = () => {
  /**
   * Persist a decrypted message locally.
   * The `plaintext` field is stored; ciphertext is intentionally dropped.
   */
  async function saveLocal(msg: Message & { plaintext?: string }) {
    const db = await getDb()
    await db.put('messages', { ...msg })
  }

  /**
   * Load messages for a conversation from local IndexedDB.
   * Already decrypted — no server call needed.
   */
  async function loadLocal(conversationId: string, limit = 50): Promise<Message[]> {
    const db = await getDb()
    const all: Message[] = await db.getAllFromIndex('messages', 'by_conversation', conversationId)
    return all
      .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
      .slice(0, limit)
      .reverse()
  }

  /**
   * Delete local messages for a conversation (e.g. on leave).
   */
  async function deleteLocalConversation(conversationId: string) {
    const db = await getDb()
    const msgs: Message[] = await db.getAllFromIndex('messages', 'by_conversation', conversationId)
    const tx = db.transaction('messages', 'readwrite')
    await Promise.all(msgs.map(m => tx.store.delete(m.id)))
    await tx.done
  }

  /**
   * Search local messages by keyword (client-side full-text).
   */
  async function searchLocal(keyword: string, limit = 30): Promise<Message[]> {
    const db = await getDb()
    const all: Message[] = await db.getAll('messages')
    const lower = keyword.toLowerCase()
    return all
      .filter(m => m.plaintext?.toLowerCase().includes(lower))
      .slice(0, limit)
  }

  return { saveLocal, loadLocal, deleteLocalConversation, searchLocal }
}
