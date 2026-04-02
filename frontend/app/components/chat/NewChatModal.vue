<template>
  <!-- Modal Vue Native - Middle Center -->
  <Teleport to="body">
    <div
      v-if="modelValue"
      class="fixed inset-0 z-50 flex items-center justify-center p-4"
      @click.self="closeModal"
    >
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-dark-950/80 backdrop-blur-sm transition-opacity"></div>

      <!-- Modal Content - FlyonUI Middle Center Style -->
      <div class="relative bg-dark-900 border border-dark-800 rounded-lg shadow-2xl w-full max-w-md overflow-hidden">
        <!-- Header -->
        <div class="modal-header flex items-center justify-between p-6 border-b border-dark-800">
          <h3 class="modal-title font-display text-xl font-bold text-text-primary">{{ t('chat.newModal.title') }}</h3>
          <button
            type="button"
            class="btn btn-text btn-circle btn-sm absolute end-3 top-3"
            aria-label="Close"
            @click="closeModal"
          >
            <span class="icon-[lucide--x] size-4"></span>
          </button>
        </div>

        <!-- Body -->
        <div class="modal-body p-6 space-y-4">
          <!-- Email Input -->
          <div class="form-control">
            <label class="label-text mb-2 block text-text-secondary">{{ t('chat.newModal.addByEmail') }}</label>
            <div class="input input-bordered border-dark-700 bg-dark-950 flex items-center gap-3 px-4 py-3">
              <span class="icon-[lucide--mail] size-5 text-text-muted shrink-0"></span>
              <input
                ref="emailInputRef"
                v-model="emailInput"
                type="email"
                :placeholder="t('chat.newModal.emailPlaceholder')"
                class="bg-transparent border-0 outline-none w-full text-text-primary placeholder:text-text-muted"
                @keyup.enter="addEmail"
              />
              <button
                class="btn btn-primary btn-sm shrink-0"
                @click="addEmail"
                :disabled="!emailInput.trim() || loading"
              >
                <span v-if="loading" class="loading loading-spinner loading-xs"></span>
                <span v-else class="icon-[lucide--plus] size-4"></span>
              </button>
            </div>
          </div>

          <!-- Selected User Badge -->
          <div v-if="selectedEmail" class="flex flex-wrap gap-2">
            <div class="badge badge-primary badge-soft gap-2 px-3 py-2">
              <span class="icon-[lucide--user-check] size-4"></span>
              {{ selectedEmail }}
            </div>
          </div>

          <!-- Error Alert -->
          <div v-if="errorMessage" class="alert alert-soft alert-error">
            <span class="icon-[lucide--alert-circle] size-5 shrink-0"></span>
            <span class="text-sm">{{ errorMessage }}</span>
          </div>
        </div>

        <!-- Footer -->
        <div class="modal-footer flex justify-end gap-2 p-6 border-t border-dark-800">
          <button class="btn btn-ghost btn-sm" @click="closeModal">
            {{ t('chat.newModal.cancel') }}
          </button>
          <button
            class="btn btn-primary btn-sm"
            :disabled="!selectedUserId || creating"
            @click="handleStartConversation"
          >
            <span v-if="creating" class="loading loading-spinner loading-xs"></span>
            <span v-else class="flex items-center gap-2">
              <span class="icon-[lucide--message-circle-plus] size-4"></span>
              {{ t('chat.newModal.startConversation') }}
            </span>
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const { authFetch } = useAuth()
const router = useRouter()
const convStore = useConversationStore()
const chat = useChat()
const { t } = useI18n()

const emailInputRef = ref<HTMLInputElement>()
const emailInput = ref('')
const selectedEmail = ref('')
const selectedUserId = ref('')
const loading = ref(false)
const creating = ref(false)
const errorMessage = ref('')

// Focus input when modal opens
watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    // Reset state
    emailInput.value = ''
    selectedEmail.value = ''
    selectedUserId.value = ''
    errorMessage.value = ''

    // Focus input after modal opens
    nextTick(() => {
      emailInputRef.value?.focus()
    })
  }
})

async function addEmail() {
  const email = emailInput.value.trim()
  if (!email || !email.includes('@')) {
    errorMessage.value = t('chat.newModal.errors.invalidEmail')
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
    errorMessage.value = e.data?.message || e.message || t('chat.newModal.errors.userNotFound')
  } finally {
    loading.value = false
  }
}

function closeModal() {
  emit('update:modelValue', false)
}

async function handleStartConversation() {
  if (!selectedUserId.value) {
    errorMessage.value = t('chat.newModal.errors.selectUserFirst')
    return
  }
  await createConversation()
}

async function createConversation() {
  if (!selectedUserId.value) {
    errorMessage.value = t('chat.newModal.errors.noUserSelected')
    return
  }

  creating.value = true
  errorMessage.value = ''
  try {
    // useChat.createConversation: creates the convo + pre-warms the E2E session key
    const conv = await chat.createConversation({
      participantUserId: selectedUserId.value,
    })
    closeModal()
    await router.push(`/conversations/${conv.id}`)
  } catch (e: any) {
    errorMessage.value = e.data?.message || e.message || t('chat.newModal.errors.createFailed')
  } finally {
    creating.value = false
  }
}
</script>
