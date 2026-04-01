export type ToastVariant = 'primary' | 'secondary' | 'info' | 'success' | 'warning' | 'error'

export interface ToastItem {
  id: string
  message: string
  variant: ToastVariant
  duration: number
  closable: boolean
}

interface ToastOptions {
  variant?: ToastVariant
  duration?: number
  closable?: boolean
}

const DEFAULT_DURATION = 5000

export const useToast = () => {
  const toasts = useState<ToastItem[]>('ui:toasts', () => [])

  function remove(id: string) {
    toasts.value = toasts.value.filter(t => t.id !== id)
  }

  function push(message: string, options: ToastOptions = {}): string {
    const id = crypto.randomUUID()
    const toast: ToastItem = {
      id,
      message,
      variant: options.variant ?? 'info',
      duration: options.duration ?? DEFAULT_DURATION,
      closable: options.closable ?? true,
    }

    toasts.value = [...toasts.value, toast]

    if (toast.duration > 0 && import.meta.client) {
      setTimeout(() => remove(id), toast.duration)
    }

    return id
  }

  function primary(message: string, options: Omit<ToastOptions, 'variant'> = {}) {
    return push(message, { ...options, variant: 'primary' })
  }

  function secondary(message: string, options: Omit<ToastOptions, 'variant'> = {}) {
    return push(message, { ...options, variant: 'secondary' })
  }

  function info(message: string, options: Omit<ToastOptions, 'variant'> = {}) {
    return push(message, { ...options, variant: 'info' })
  }

  function success(message: string, options: Omit<ToastOptions, 'variant'> = {}) {
    return push(message, { ...options, variant: 'success' })
  }

  function warning(message: string, options: Omit<ToastOptions, 'variant'> = {}) {
    return push(message, { ...options, variant: 'warning' })
  }

  function error(message: string, options: Omit<ToastOptions, 'variant'> = {}) {
    return push(message, { ...options, variant: 'error' })
  }

  function clear() {
    toasts.value = []
  }

  return {
    toasts,
    push,
    remove,
    clear,
    primary,
    secondary,
    info,
    success,
    warning,
    error,
  }
}
