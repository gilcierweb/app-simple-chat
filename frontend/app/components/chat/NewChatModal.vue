<template>
  <div class="modal modal-open" role="dialog">
    <div class="modal-box max-w-md bg-dark-900 border border-dark-800">
      <!-- Header -->
      <div class="flex items-center justify-between mb-4">
        <h3 class="font-display text-xl font-bold text-text-primary">New conversation</h3>
        <button class="btn btn-ghost btn-sm btn-square" @click="$emit('close')">
          <span class="icon-[lucide--x] size-5"></span>
        </button>
      </div>

      <!-- Tabs -->
      <div class="tabs tabs-bordered mb-4">
        <button 
          class="tab" 
          :class="{ 'tab-active text-primary': mode === 'direct' }" 
          @click="mode = 'direct'"
        >
          <span class="icon-[lucide--user] size-4 mr-1"></span>
          Direct
        </button>
        <button 
          class="tab" 
          :class="{ 'tab-active text-primary': mode === 'group' }" 
          @click="mode = 'group'"
        >
          <span class="icon-[lucide--users] size-4 mr-1"></span>
          Group
        </button>
      </div>

      <!-- Group Name -->
      <div v-if="mode === 'group'" class="form-control mb-3">
        <label class="label"><span class="label-text text-text-secondary">Group name</span></label>
        <label class="input-group input-group-sm">
          <span class="input-group-text">
            <span class="icon-[lucide--type] size-4 text-text-muted"></span>
          </span>
          <input 
            v-model="groupName" 
            type="text" 
            placeholder="My awesome group" 
            class="input input-filled flex-1" 
          />
        </label>
      </div>

      <!-- Add Member -->
      <div class="form-control mb-4">
        <label class="label"><span class="label-text text-text-secondary">Add by email</span></label>
        <label class="input-group input-group-sm">
          <span class="input-group-text">
            <span class="icon-[lucide--mail] size-4 text-text-muted"></span>
          </span>
          <input
            v-model="emailInput"
            type="email"
            placeholder="contact@example.com"
            class="input input-filled flex-1"
            @keydown.enter.prevent="addEmail"
          />
          <button class="btn btn-primary btn-sm" @click="addEmail">
            <span class="icon-[lucide--plus] size-4"></span>
          </button>
        </label>
      </div>

      <!-- Selected Members -->
      <div v-if="members.length > 0" class="flex flex-wrap gap-2 mb-4">
        <div 
          v-for="m in members" 
          :key="m" 
          class="badge badge-primary badge-soft gap-1 cursor-pointer hover:bg-primary/20"
          @click="removeMember(m)"
        >
          <span class="icon-[lucide--mail] size-3"></span>
          {{ m }}
          <span class="icon-[lucide--x] size-3"></span>
        </div>
      </div>

      <!-- Error Alert -->
      <div v-if="error" class="alert alert-soft alert-error text-sm py-2 mb-3">
        <span class="icon-[lucide--alert-circle] size-4"></span>
        {{ error }}
      </div>

      <!-- Actions -->
      <div class="modal-action mt-2">
        <button class="btn btn-ghost btn-sm" @click="$emit('close')">Cancel</button>
        <button
          class="btn btn-primary btn-sm"
          :disabled="members.length === 0 || creating"
          @click="create"
        >
          <span v-if="creating" class="loading loading-spinner loading-xs"></span>
          <span v-else class="flex items-center gap-1">
            <span class="icon-[lucide--message-circle-plus] size-4"></span>
            Start conversation
          </span>
        </button>
      </div>
    </div>
    <div class="modal-backdrop bg-dark-950/80" @click="$emit('close')"></div>
  </div>
</template>

<script setup lang="ts">
import { useConversationStore } from '~/stores/conversations'

const emit = defineEmits<{ close: [] }>()
const { authFetch } = useAuth()
const router = useRouter()
const convStore = useConversationStore()

const mode = ref<'direct' | 'group'>('direct')
const groupName = ref('')
const emailInput = ref('')
const members = ref<string[]>([])
const creating = ref(false)
const error = ref('')

function addEmail() {
  const e = emailInput.value.trim()
  if (!e || !e.includes('@')) return
  if (!members.value.includes(e)) members.value.push(e)
  emailInput.value = ''
}

function removeMember(email: string) {
  members.value = members.value.filter(m => m !== email)
}

async function create() {
  if (members.value.length === 0) return
  creating.value = true
  error.value = ''
  try {
    // In real implementation, look up user IDs by email first
    // For now, placeholder with mock UUIDs
    const conv = await authFetch<any>('/conversations', {
      method: 'POST',
      body: JSON.stringify({
        conversation_type: mode.value,
        member_ids: [],  // TODO: resolve email → UUID
        name: mode.value === 'group' ? groupName.value : undefined,
      }),
    })
    convStore.upsertConversation(conv)
    emit('close')
    router.push(`/conversations/${conv.id}`)
  } catch (e: any) {
    error.value = e.message || 'Failed to create conversation'
  } finally {
    creating.value = false
  }
}
</script>
