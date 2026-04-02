// Composable para inicializar FlyonUI
export function useFlyonUI() {
  const initFlyonUI = async () => {
    if (typeof window === 'undefined') return
    
    // Dynamic import flyonui
    await import('flyonui/flyonui')
    
    // Initialize after a short delay to ensure DOM is ready
    setTimeout(() => {
      if (window.HSStaticMethods) {
        window.HSStaticMethods.autoInit()
      }
      if (window.HSOverlay) {
        window.HSOverlay.autoInit()
      }
    }, 100)
  }
  
  const openModal = (selector: string) => {
    if (typeof window !== 'undefined' && window.HSOverlay) {
      window.HSOverlay.open(selector)
    } else {
      // Fallback manual
      const modal = document.querySelector(selector)
      if (modal) {
        modal.classList.remove('hidden')
        modal.classList.add('open')
      }
    }
  }
  
  const closeModal = (selector: string) => {
    if (typeof window !== 'undefined' && window.HSOverlay) {
      window.HSOverlay.close(selector)
    } else {
      // Fallback manual
      const modal = document.querySelector(selector)
      if (modal) {
        modal.classList.add('hidden')
        modal.classList.remove('open')
      }
    }
  }
  
  return {
    initFlyonUI,
    openModal,
    closeModal
  }
}
