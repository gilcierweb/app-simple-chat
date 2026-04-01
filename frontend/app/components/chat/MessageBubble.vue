<template>
  <div :class="['chat', isOwn ? 'chat-sender' : 'chat-receiver', 'group relative']">
    <!-- Avatar -->
    <div v-if="showAvatar" class="chat-avatar avatar">
      <div class="size-10 rounded-full bg-brand-600 flex items-center justify-center text-sm font-bold text-white">
        {{ senderInitial }}
      </div>
    </div>
    <div v-else class="chat-avatar avatar opacity-0">
      <div class="size-10"></div>
    </div>

    <!-- Header with sender info and time -->
    <div class="chat-header text-text-secondary">
      <span v-if="!isOwn" class="font-medium text-text-primary">{{ senderName }}</span>
      <time class="text-text-muted">{{ formatTime(message.created_at) }}</time>
    </div>

    <!-- Chat bubble -->
    <div class="chat-bubble" :class="{ 'opacity-50': message.deleted_at }">
      <!-- Deleted message -->
      <template v-if="message.deleted_at">
        <span class="italic text-text-muted">{{ t('chat.message.deleted') }}</span>
      </template>

      <!-- Reply reference -->
      <div v-else-if="message.reply_to_id" class="text-xs text-text-muted mb-2 px-2 border-l-2 border-dark-700 truncate max-w-full">
        {{ t('chat.message.replyingTo') }}
      </div>

      <!-- Text content -->
      <template v-else-if="message.message_type === 'text' || !message.message_type">
        <span class="whitespace-pre-wrap break-words">{{ message.plaintext || t('chat.message.encryptedWithIcon') }}</span>
      </template>

      <!-- Image -->
      <template v-else-if="message.message_type === 'image'">
        <div class="flex flex-col gap-2">
          <button class="border-dark-700 overflow-hidden rounded-md border" :aria-label="t('chat.message.image')">
            <img class="w-48 h-auto object-cover" src="https://cdn.flyonui.com/fy-assets/components/card/image-9.png" :alt="t('chat.message.sharedImage')" />
          </button>
        </div>
      </template>

      <!-- File -->
      <template v-else-if="message.message_type === 'file'">
        <div class="bg-dark-800 rounded-md">
          <button class="flex items-center gap-2 px-3 py-2 max-sm:w-52">
            <div class="flex flex-col gap-1 max-sm:w-5/6">
              <div class="flex items-center">
                <span class="icon-[lucide--file-text] text-primary me-2 size-5"></span>
                <span class="text-text-secondary truncate font-medium text-sm">document.pdf</span>
              </div>
              <div class="text-text-muted flex items-center gap-1 text-xs max-sm:hidden">
                2.4 MB
                <span class="icon-[lucide--circle] mt-0.5 size-1 fill-current"></span>
                PDF
              </div>
            </div>
            <span class="btn btn-ghost btn-circle btn-sm">
              <span class="icon-[lucide--download] size-4"></span>
            </span>
          </button>
        </div>
      </template>
    </div>

    <!-- Footer with status -->
    <div v-if="isOwn" class="chat-footer text-text-muted">
      {{ statusText }}
      <span v-if="statusIcon" :class="[statusIcon, statusColor, 'align-bottom size-4 inline-block ml-1']"></span>
    </div>

    <!-- Context menu (hover) -->
    <div class="absolute top-0 opacity-0 group-hover:opacity-100 transition-opacity z-10"
         :class="isOwn ? 'left-0 -translate-x-full pr-1' : 'right-0 translate-x-full pl-1'">
      <div class="flex items-center gap-0.5 bg-dark-900 shadow-lg rounded-lg p-0.5 border border-dark-800">
        <button class="btn btn-ghost btn-xs btn-square" :title="t('chat.message.react')">😊</button>
        <button class="btn btn-ghost btn-xs btn-square" :title="t('chat.message.reply')" @click="$emit('reply', message)">
          <span class="icon-[lucide--reply] size-4"></span>
        </button>
        <button v-if="isOwn && !message.deleted_at" class="btn btn-ghost btn-xs btn-square text-error" :title="t('chat.message.delete')" @click="$emit('delete', message.id)">
          <span class="icon-[lucide--trash-2] size-4"></span>
        </button>
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
  senderName?: string
}>()

const emit = defineEmits<{
  delete: [id: string]
  reply: [msg: Message]
}>()

const senderInitial = computed(() => props.message.sender_id.slice(0, 1).toUpperCase())
const senderName = computed(() => props.senderName || props.message.sender_id.slice(0, 8))
const { t } = useI18n()

const statusText = computed(() => {
  switch (props.message.status) {
    case 'sending': return t('chat.status.sending')
    case 'sent': return t('chat.status.sent')
    case 'delivered': return t('chat.status.delivered')
    case 'read': return t('chat.status.read')
    case 'error': return t('chat.status.failed')
    default: return ''
  }
})

const statusIcon = computed(() => {
  switch (props.message.status) {
    case 'sending': return 'icon-[lucide--clock]'
    case 'sent': return 'icon-[lucide--check]'
    case 'delivered': return 'icon-[lucide--check-check]'
    case 'read': return 'icon-[lucide--check-check]'
    case 'error': return 'icon-[lucide--alert-circle]'
    default: return ''
  }
})

const statusColor = computed(() => {
  if (props.message.status === 'read') return 'text-success'
  if (props.message.status === 'error') return 'text-error'
  return ''
})

function formatTime(iso: string): string {
  return new Date(iso).toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' })
}
</script>
