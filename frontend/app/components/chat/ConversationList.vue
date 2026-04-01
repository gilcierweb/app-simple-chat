<template>
  <div>
    <!-- Empty State -->
    <div v-if="filtered.length === 0" class="px-4 py-12 text-center">
      <div class="flex flex-col items-center gap-3 text-text-muted">
        <span class="icon-[lucide--inbox] size-12 opacity-40"></span>
        <p class="text-sm">{{ search ? t('chat.list.empty.search') : t('chat.list.empty.default') }}</p>
        <p v-if="!search" class="text-xs">{{ t('chat.list.empty.hint') }}</p>
      </div>
    </div>

    <!-- Conversation Items -->
    <NuxtLink
      v-for="conv in filtered"
      :key="conv.id"
      :to="`/conversations/${conv.id}`"
      class="flex items-center gap-3 px-4 py-3 hover:bg-dark-800/50 transition-colors cursor-pointer border-b border-dark-800/50 group"
      :class="{ 'bg-primary/10 hover:bg-primary/20': activeId === conv.id }"
      @click="convStore.setActiveConversation(conv.id)"
    >
      <!-- Avatar -->
      <div class="avatar avatar-placeholder relative flex-shrink-0">
        <div class="w-11 h-11 rounded-full bg-brand-600 text-white flex items-center justify-center font-bold">
          {{ convInitial(conv) }}
        </div>
        <span
          v-if="onlineUsers.has(conv.id)"
          class="absolute bottom-0 right-0 w-3 h-3 bg-primary rounded-full border-2 border-dark-900"
        ></span>
      </div>

      <!-- Info -->
      <div class="flex-1 min-w-0">
        <div class="flex items-center justify-between">
          <span class="font-medium text-sm text-text-primary truncate">{{ convName(conv) }}</span>
          <span class="text-xs text-text-muted flex-shrink-0 ml-2">
            {{ formatLastActive(conv.updated_at) }}
          </span>
        </div>
        <div class="flex items-center justify-between mt-0.5">
          <p class="text-xs text-text-secondary truncate">
            <span v-if="conv.last_message?.deleted_at" class="italic flex items-center gap-1">
              <span class="icon-[lucide--trash-2] size-3"></span>
              {{ t('chat.message.deleted') }}
            </span>
            <span v-else-if="conv.last_message?.plaintext" class="text-text-secondary">{{ conv.last_message.plaintext }}</span>
            <span v-else class="text-text-muted flex items-center gap-1">
              <span class="icon-[lucide--lock] size-3"></span>
              {{ t('chat.message.encrypted') }}
            </span>
          </p>
          <span
            v-if="unreadCount(conv.id) > 0"
            class="badge badge-primary badge-sm flex-shrink-0 ml-2"
          >
            {{ unreadCount(conv.id) > 99 ? '99+' : unreadCount(conv.id) }}
          </span>
        </div>
      </div>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
import type { Conversation } from '~/types'
import { useConversationStore } from '~/stores/conversations'

const props = defineProps<{ search?: string }>()

const convStore = useConversationStore()
const { t } = useI18n()
const activeId = computed(() => convStore.activeConversationId)
const onlineUsers = ref(new Set<string>())

const filtered = computed(() => {
  const list = convStore.conversations
  if (!props.search?.trim()) return list
  const q = props.search.toLowerCase()
  return list.filter(c => convName(c).toLowerCase().includes(q))
})

function convInitial(c: Conversation): string {
  return (convName(c)[0] ?? '?').toUpperCase()
}

function convName(c: Conversation): string {
  return c.name ?? (c.conversation_type === 'direct'
    ? t('chat.common.directMessage')
    : t('chat.common.group'))
}

function formatLastActive(iso: string): string {
  const d = new Date(iso)
  const now = new Date()
  const diff = now.getTime() - d.getTime()
  if (diff < 60_000) return t('chat.list.time.now')
  if (diff < 3_600_000) return t('chat.list.time.minutes', { count: Math.floor(diff / 60_000) })
  if (diff < 86_400_000) return t('chat.list.time.hours', { count: Math.floor(diff / 3_600_000) })
  return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
}

function unreadCount(convId: string): number {
  // Count messages after last_read_at for this conversation
  const msgs = convStore.messages[convId] ?? []
  const conv = convStore.conversations.find(c => c.id === convId)
  const lastRead = conv?.members?.[0]?.last_read_at
  if (!lastRead) return msgs.length
  return msgs.filter(m => new Date(m.created_at) > new Date(lastRead)).length
}
</script>
