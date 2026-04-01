/**
 * Extrai mensagem de erro de uma resposta de API
 * Suporta múltiplos formatos de erro comuns
 */
export function getApiErrorMessage(error: any, t: (key: string) => string, fallbackKey?: string): string {
  // Tenta extrair mensagem de vários formatos possíveis
  const message = error?.data?.message
    || error?.data?.error?.message  // error.error é objeto com message
    || error?.message
    || error?.statusMessage
    || error?.statusText

  if (message && typeof message === 'string') {
    return message
  }

  // Se tiver um array de erros (validação), pega o primeiro
  if (error?.data?.errors && Array.isArray(error.data.errors)) {
    return error.data.errors[0]?.message || error.data.errors[0]
  }

  // Fallback para chave de tradução ou mensagem genérica
  if (fallbackKey) {
    return t(fallbackKey)
  }

  return t('common.errors.generic')
}

/**
 * Helper para mostrar alerta de erro da API
 */
export function useApiAlert() {
  const alert = useAlert()
  const { t } = useI18n()

  function showError(error: any, fallbackKey?: string) {
    const message = getApiErrorMessage(error, t, fallbackKey)
    alert.error(message)
  }

  function showSuccess(message: string) {
    alert.success(message)
  }

  function showInfo(message: string) {
    alert.info(message)
  }

  function showWarning(message: string) {
    alert.warning(message)
  }

  return {
    showError,
    showSuccess,
    showInfo,
    showWarning
  }
}
