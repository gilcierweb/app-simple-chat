<template>
  <div class="flex flex-col items-center justify-center h-full p-8">
    <div class="card bg-dark-900 border border-dark-800 max-w-md w-full">
      <div class="card-body items-center text-center p-12">
        <!-- Icon -->
        <div class="flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 mb-6">
          <span class="icon-[lucide--message-circle] size-10 text-primary"></span>
        </div>

        <!-- Title -->
        <h3 class="card-title text-text-primary text-xl">{{ t('chat.index.title') }}</h3>
        <p class="text-text-secondary mt-2">
          {{ t('chat.index.subtitle') }}
        </p>

        <!-- Action -->
        <button class="btn btn-primary btn-soft mt-6" @click="handleNewChat">
          <span class="icon-[lucide--plus] size-4"></span>
          {{ t('chat.index.startConversation') }}
        </button>
      </div>
    </div>

    <!-- Security tip -->
    <div class="mt-8 flex items-center gap-2 text-xs text-text-muted">
      <span class="icon-[lucide--lock] size-4 text-primary"></span>
      <span>{{ t('chat.index.securityTip') }}</span>
    </div>
  </div>

  <!-- Modals FlyonUI -->
  <!-- Bottom Start -->
  <div id="bottom-start-modal" class="overlay modal overlay-open:opacity-100 overlay-open:duration-300 modal-bottom-start hidden" role="dialog" tabindex="-1">
    <div class="modal-dialog">
      <div class="modal-content bg-dark-900 border border-dark-800">
        <div class="modal-header border-b border-dark-800">
          <h3 class="modal-title text-text-primary">Dialog Title</h3>
          <button type="button" class="btn btn-text btn-circle btn-sm absolute end-3 top-3" aria-label="Close" data-overlay="#bottom-start-modal">
            <span class="icon-[lucide--x] size-4"></span>
          </button>
        </div>
        <div class="modal-body text-text-secondary">
          This is some placeholder content to show the scrolling behavior for modals.
        </div>
        <div class="modal-footer border-t border-dark-800 gap-2">
          <button type="button" class="btn btn-soft btn-secondary" data-overlay="#bottom-start-modal">Close</button>
          <button type="button" class="btn btn-primary">Save changes</button>
        </div>
      </div>
    </div>
  </div>

  <!-- Bottom Center -->
  <div id="bottom-center-modal" class="overlay modal overlay-open:opacity-100 overlay-open:duration-300 modal-bottom hidden" role="dialog" tabindex="-1">
    <div class="modal-dialog">
      <div class="modal-content bg-dark-900 border border-dark-800">
        <div class="modal-header border-b border-dark-800">
          <h3 class="modal-title text-text-primary">Dialog Title</h3>
          <button type="button" class="btn btn-text btn-circle btn-sm absolute end-3 top-3" aria-label="Close" data-overlay="#bottom-center-modal">
            <span class="icon-[lucide--x] size-4"></span>
          </button>
        </div>
        <div class="modal-body text-text-secondary">
          This is some placeholder content to show the scrolling behavior for modals.
        </div>
        <div class="modal-footer border-t border-dark-800 gap-2">
          <button type="button" class="btn btn-soft btn-secondary" data-overlay="#bottom-center-modal">Close</button>
          <button type="button" class="btn btn-primary">Save changes</button>
        </div>
      </div>
    </div>
  </div>

  <!-- Bottom End -->
  <div id="bottom-end-modal" class="overlay modal overlay-open:opacity-100 overlay-open:duration-300 modal-bottom-end hidden" role="dialog" tabindex="-1">
    <div class="modal-dialog">
      <div class="modal-content bg-dark-900 border border-dark-800">
        <div class="modal-header border-b border-dark-800">
          <h3 class="modal-title text-text-primary">Dialog Title</h3>
          <button type="button" class="btn btn-text btn-circle btn-sm absolute end-3 top-3" aria-label="Close" data-overlay="#bottom-end-modal">
            <span class="icon-[lucide--x] size-4"></span>
          </button>
        </div>
        <div class="modal-body text-text-secondary">
          This is some placeholder content to show the scrolling behavior for modals.
        </div>
        <div class="modal-footer border-t border-dark-800 gap-2">
          <button type="button" class="btn btn-soft btn-secondary" data-overlay="#bottom-end-modal">Close</button>
          <button type="button" class="btn btn-primary">Save changes</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ 
  layout: 'default',
  requiresAuth: true,
})
const { t } = useI18n()

const authStore = useAuthStore()
const { authFetch } = useAuth()
const convStore = useConversationStore()

function handleNewChat() {
  convStore.openNewChatModal()
}

// Load conversations on mount
onMounted(async () => {
  // Initialize FlyonUI modals
  if (typeof window !== 'undefined' && window.HSOverlay) {
    window.HSOverlay.autoInit()
  }
  
  try {
    const data = await authFetch<any[]>('/conversations')
    convStore.setConversations(data)
  } catch {}
})
</script>
