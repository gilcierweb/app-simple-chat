<template>
  <!-- FlyonUI Modal Overlay -->
  <div
    v-if="isOpen"
    class="overlay modal modal-middle"
    :class="{ 'overlay-open:opacity-100 opacity-100': isOpen }"
    role="dialog"
    tabindex="-1"
    @click.self="closeModal"
  >
    <div class="modal-dialog">
      <div class="modal-content bg-dark-900 border border-dark-800">
        <!-- Header -->
        <div class="modal-header border-b border-dark-800">
          <h3 class="modal-title text-text-primary">{{ t('chat.newModal.title') }}</h3>
          <button
            type="button"
            class="btn btn-ghost btn-circle btn-sm absolute end-3 top-3"
            aria-label="Close"
            @click="closeModal"
          >
            <span class="icon-[lucide--x] size-4"></span>
          </button>
        </div>

        <!-- Body -->
        <div class="modal-body space-y-4">
          <!-- Email Input -->
          <div class="form-control">
            <label class="label-text mb-2 block text-text-secondary">{{ t('chat.newModal.addByEmail') }}</label>
            <div class="input input-bordered border-dark-700 bg-dark-950 flex items-center gap-3 px-4 py-3">
              <span class="icon-[lucide--mail] size-5 text-text-muted shrink-0"></span>
              <input
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
        <div class="modal-footer border-t border-dark-800 gap-2">
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
  </div>

  <!-- Backdrop -->
  <div
    v-if="isOpen"
    class="fixed inset-0 bg-dark-950/80 z-[998]"
    @click="closeModal"
  ></div>
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
const { t } = useI18n()

const emailInput = ref('')
const selectedEmail = ref('')
const selectedUserId = ref('')
const loading = ref(false)
const creating = ref(false)
const errorMessage = ref('')

const isOpen = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
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
  isOpen.value = false
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
    const conv = await authFetch<any>('/conversations', {
      method: 'POST',
      body: {
        participant_user_id: selectedUserId.value,
        conversation_type: 1,
      } as any,
    })
    convStore.upsertConversation(conv as any)
    closeModal()
    await router.push(`/conversations/${conv.id}`)
  } catch (e: any) {
    errorMessage.value = e.data?.message || e.message || t('chat.newModal.errors.createFailed')
  } finally {
    creating.value = false
  }
}

// Reset state when modal opens
watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    emailInput.value = ''
    selectedEmail.value = ''
    selectedUserId.value = ''
    errorMessage.value = ''
  }
})
</script>
