<template>
  <div class="modal-overlay" @click.self="closeModal">
    <div class="modal-box">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-xl font-bold">New conversation</h3>
        <button class="btn btn-ghost btn-sm btn-square" @click="closeModal">
          <span class="icon-[lucide--x] size-5"></span>
        </button>
      </div>

      <div class="form-control mb-4">
        <label class="label"><span class="label-text">Add by email</span></label>
        <div class="input input-bordered flex items-center gap-2 p-0">
          <span class="icon-[tabler--mail] text-base-content/70 ms-3 size-5 shrink-0"></span>
          <input
            v-model="emailInput"
            type="email"
            placeholder="contact@example.com"
            class="grow bg-transparent border-none focus:ring-0 py-2"
            @keyup.enter="addEmail"
          />
          <button class="btn btn-primary btn-sm me-1" @click="addEmail" :disabled="!emailInput.trim() || loading">
            <span v-if="loading" class="loading loading-spinner loading-xs"></span>
            <span v-else class="icon-[tabler--plus] size-4"></span>
          </button>
        </div>
      </div>

      <div v-if="selectedEmail" class="flex flex-wrap gap-2 mb-4">
        <div class="badge badge-primary badge-soft gap-1">
          <span class="icon-[tabler--mail] size-3"></span>
          {{ selectedEmail }}
        </div>
      </div>

      <div v-if="errorMessage" class="alert alert-soft alert-error text-sm py-2 mb-3">
        <span class="icon-[tabler--alert-circle] size-4"></span>
        {{ errorMessage }}
      </div>

      <div class="modal-action mt-2">
        <button
          class="btn btn-primary btn-sm"
          :disabled="!selectedUserId || creating"
          @click="handleStartConversation"
        >
          <span v-if="creating" class="loading loading-spinner loading-xs"></span>
          <span v-else class="flex items-center gap-1">
            <span class="icon-[tabler--message-circle-plus] size-4"></span>
            Start conversation
          </span>
        </button>
        <button class="btn btn-ghost btn-sm" @click="closeModal">Cancel</button>
      </div>
    </div>
    <div class="modal-backdrop" @click="closeModal"></div>
  </div>
</template>

<script setup lang="ts">
const emit = defineEmits(['close'])
const { authFetch } = useAuth()
const router = useRouter()
const convStore = useConversationStore()

const emailInput = ref('')
const selectedEmail = ref('')
const selectedUserId = ref('')
const loading = ref(false)
const creating = ref(false)
const errorMessage = ref('')

async function addEmail() {
  const email = emailInput.value.trim()
  if (!email || !email.includes('@')) {
    errorMessage.value = 'Invalid email address'
    return
  }
  
  loading.value = true
  errorMessage.value = ''
  selectedEmail.value = ''
  selectedUserId.value = ''
  try {
    const user = await authFetch<{user_id: string, email: string, conversation_type: string}>('/users/lookup?email=' + encodeURIComponent(email))
    if (user && user.user_id) {
      selectedEmail.value = user.email
      selectedUserId.value = user.user_id
    }
  } catch (e: any) {
    errorMessage.value = e.data?.message || e.message || 'User not found'
  } finally {
    loading.value = false
  }
}

function closeModal() {
  emit('close')
}

async function handleStartConversation() {
  if (!selectedUserId.value) {
    errorMessage.value = 'Please select a user first'
    return
  }
  await createConversation()
}

async function createConversation() {
  if (!selectedUserId.value) {
    errorMessage.value = 'No user selected'
    return
  }
  
  creating.value = true
  errorMessage.value = ''
  try {
    const conv = await authFetch<any>('/conversations', {
      method: 'POST',
      body: {
        participant_user_id: selectedUserId.value,
        conversation_type: 1,
      },
    })
    console.log('[DEBUG] Created conversation:', conv, 'members:', conv?.members)
    convStore.upsertConversation(conv as any)
    emit('close')
    await router.push(`/conversations/${conv.id}`)
  } catch (e: any) {
    errorMessage.value = e.data?.message || e.message || 'Failed to create conversation'
  } finally {
    creating.value = false
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.modal-box {
  background: #1a1a2e;
  border: 1px solid #333;
  padding: 24px;
  border-radius: 12px;
  max-width: 450px;
  width: 100%;
  color: #fff;
  position: relative;
  z-index: 10000;
}

.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
}
</style>