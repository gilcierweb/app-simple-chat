<template>
  <div
    class="flex items-end gap-2"
    :class="isOwn ? 'flex-row-reverse' : 'flex-row'"
  >
    <!-- Avatar (other person) -->
    <div v-if="!isOwn && showAvatar" class="w-8 h-8 rounded-full bg-secondary flex-shrink-0 flex items-center justify-center text-xs font-bold text-secondary-content">
      {{ senderInitial }}
    </div>
    <div v-else-if="!isOwn" class="w-8 flex-shrink-0"></div>

    <!-- Bubble -->
    <div
      class="group relative flex flex-col"
      :class="isOwn ? 'items-end' : 'items-start'"
      style="max-width: 70%"
    >
      <!-- Reply reference -->
      <div v-if="message.reply_to_id" class="text-xs text-base-content/40 mb-1 px-2 border-l-2 border-base-300 truncate max-w-full">
        Replying to message
      </div>

      <div
        class="px-3 py-2 text-sm relative"
        :class="[isOwn ? 'msg-out' : 'msg-in', { 'opacity-50': message.deleted_at }]"
      >
        <!-- Deleted message -->
        <template v-if="message.deleted_at">
          <span class="italic text-base-content/40">Message deleted</span>
        </template>

        <!-- Text content -->
        <template v-else-if="message.message_type === 'text'">
          <span class="whitespace-pre-wrap break-words">{{ message.plaintext || '🔒' }}</span>
        </template>

        <!-- Image -->
        <template v-else-if="message.message_type === 'image'">
          <span class="text-base-content/50 italic text-xs">📷 Image</span>
        </template>

        <!-- File -->
        <template v-else-if="message.message_type === 'file'">
          <span class="text-base-content/50 italic text-xs">📎 File attachment</span>
        </template>

        <!-- Timestamp + status -->
        <div class="flex items-center gap-1 justify-end mt-1">
          <span class="text-xs text-base-content/40 select-none">{{ formatTime(message.created_at) }}</span>
          <span v-if="isOwn" class="text-xs" :class="statusColor">{{ statusIcon }}</span>
        </div>
      </div>

      <!-- Context menu (hover) -->
      <div class="absolute top-0 opacity-0 group-hover:opacity-100 transition-opacity z-10"
           :class="isOwn ? 'left-0 -translate-x-full pr-1' : 'right-0 translate-x-full pl-1'">
        <div class="flex items-center gap-0.5 bg-base-100 shadow rounded-lg p-0.5 border border-base-200">
          <button class="btn btn-ghost btn-xs btn-square" title="React">😊</button>
          <button class="btn btn-ghost btn-xs btn-square" title="Reply" @click="$emit('reply', message)">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
            </svg>
          </button>
          <button v-if="isOwn && !message.deleted_at" class="btn btn-ghost btn-xs btn-square text-error" title="Delete" @click="$emit('delete', message.id)">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Message } from '~/types'

const props = defineProps<{
  message: Message
  isOwn: boolean
  showAvatar?: boolean
}>()

defineEmits<{
  delete: [id: string]
  reply: [msg: Message]
}>()

const senderInitial = computed(() => props.message.sender_id.slice(0, 1).toUpperCase())

const statusIcon = computed(() => {
  switch (props.message.status) {
    case 'sending': return '🕐'
    case 'sent': return '✓'
    case 'delivered': return '✓✓'
    case 'read': return '✓✓'
    case 'error': return '!'
    default: return ''
  }
})

const statusColor = computed(() => {
  if (props.message.status === 'read') return 'text-info'
  if (props.message.status === 'error') return 'text-error'
  return 'text-base-content/40'
})

function formatTime(iso: string): string {
  return new Date(iso).toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' })
}
</script>
