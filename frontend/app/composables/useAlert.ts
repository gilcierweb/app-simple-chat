import { ref, shallowRef } from 'vue'

export interface AlertOptions {
  variant?: 'solid' | 'soft' | 'outline' | 'dashed'
  color?: 'primary' | 'secondary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  icon?: string
  title?: string
  message: string
  duration?: number
  dismissible?: boolean
}

interface AlertItem extends AlertOptions {
  id: string
  createdAt: number
}

const alerts = shallowRef<AlertItem[]>([])

function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`
}

function addAlert(options: AlertOptions): string {
  const id = generateId()
  const alert: AlertItem = {
    id,
    variant: 'soft',
    color: 'primary',
    dismissible: true,
    duration: 5000,
    ...options,
    createdAt: Date.now()
  }
  
  alerts.value = [...alerts.value, alert]
  
  // Auto-dismiss after duration
  if (alert.duration && alert.duration > 0) {
    setTimeout(() => {
      removeAlert(id)
    }, alert.duration)
  }
  
  return id
}

function removeAlert(id: string) {
  alerts.value = alerts.value.filter(a => a.id !== id)
}

function clearAll() {
  alerts.value = []
}

// Convenience methods
function success(message: string, title?: string, options?: Partial<AlertOptions>) {
  return addAlert({
    color: 'success',
    icon: 'icon-[lucide--check-circle]',
    message,
    title,
    ...options
  })
}

function error(message: string, title?: string, options?: Partial<AlertOptions>) {
  return addAlert({
    color: 'error',
    icon: 'icon-[lucide--circle-x]',
    message,
    title,
    duration: 8000,
    ...options
  })
}

function warning(message: string, title?: string, options?: Partial<AlertOptions>) {
  return addAlert({
    color: 'warning',
    icon: 'icon-[lucide--alert-triangle]',
    message,
    title,
    ...options
  })
}

function info(message: string, title?: string, options?: Partial<AlertOptions>) {
  return addAlert({
    color: 'info',
    icon: 'icon-[lucide--info]',
    message,
    title,
    ...options
  })
}

export function useAlert() {
  return {
    alerts: readonly(alerts),
    add: addAlert,
    remove: removeAlert,
    clear: clearAll,
    success,
    error,
    warning,
    info
  }
}
