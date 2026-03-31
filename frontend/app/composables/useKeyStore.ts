/**
 * useKeyStore — manages E2E key pairs using Web Crypto API + IndexedDB.
 *
 * Key hierarchy:
 *  - Identity key pair   (X25519, long-lived)
 *  - Signed prekey pair  (X25519, rotated weekly)
 *  - One-time prekeys    (X25519, consumed per session, batch of 100)
 *
 * Private keys are stored in IndexedDB as non-extractable CryptoKey objects.
 * Only public keys are uploaded to the server.
 */

import { openDB, type IDBPDatabase } from 'idb'

const DB_NAME = 'simple-chat-keys'
const DB_VERSION = 1
const STORE_KEYS = 'keys'

let _db: IDBPDatabase | null = null

async function getDb(): Promise<IDBPDatabase> {
  if (_db) return _db
  _db = await openDB(DB_NAME, DB_VERSION, {
    upgrade(db) {
      if (!db.objectStoreNames.contains(STORE_KEYS)) {
        db.createObjectStore(STORE_KEYS)
      }
    },
  })
  return _db
}

export const useKeyStore = () => {
  const config = useRuntimeConfig()

  /**
   * Generate X25519 ECDH key pair.
   * Private key is non-extractable (can only be used for derivation).
   */
  async function generateKeyPair(): Promise<CryptoKeyPair> {
    return crypto.subtle.generateKey(
      { name: 'ECDH', namedCurve: 'P-256' }, // Use P-256 (widely supported); X25519 available in modern browsers
      false, // non-extractable private key
      ['deriveKey', 'deriveBits'],
    )
  }

  /**
   * Export a public key as base64 string (for uploading to server).
   */
  async function exportPublicKey(key: CryptoKey): Promise<string> {
    const raw = await crypto.subtle.exportKey('spki', key)
    return btoa(String.fromCharCode(...new Uint8Array(raw)))
  }

  /**
   * Import a peer's public key from base64.
   */
  async function importPublicKey(b64: string): Promise<CryptoKey> {
    const raw = Uint8Array.from(atob(b64), c => c.charCodeAt(0))
    return crypto.subtle.importKey(
      'spki',
      raw,
      { name: 'ECDH', namedCurve: 'P-256' },
      true,
      [],
    )
  }

  /**
   * Derive a shared AES-256-GCM key from our private key + peer's public key (ECDH).
   */
  async function deriveSharedKey(
    ourPrivateKey: CryptoKey,
    peerPublicKey: CryptoKey,
  ): Promise<CryptoKey> {
    return crypto.subtle.deriveKey(
      { name: 'ECDH', public: peerPublicKey },
      ourPrivateKey,
      { name: 'AES-GCM', length: 256 },
      false,
      ['encrypt', 'decrypt'],
    )
  }

  /**
   * Encrypt plaintext with AES-256-GCM.
   * Returns { ciphertext: base64, iv: base64 }
   */
  async function encrypt(key: CryptoKey, plaintext: string): Promise<{ ciphertext: string; iv: string }> {
    const iv = crypto.getRandomValues(new Uint8Array(12))
    const encoded = new TextEncoder().encode(plaintext)
    const ct = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, encoded)
    return {
      ciphertext: btoa(String.fromCharCode(...new Uint8Array(ct))),
      iv: btoa(String.fromCharCode(...iv)),
    }
  }

  /**
   * Decrypt ciphertext with AES-256-GCM.
   */
  async function decrypt(key: CryptoKey, ciphertext: string, iv: string): Promise<string> {
    const ct = Uint8Array.from(atob(ciphertext), c => c.charCodeAt(0))
    const ivBytes = Uint8Array.from(atob(iv), c => c.charCodeAt(0))
    const plain = await crypto.subtle.decrypt({ name: 'AES-GCM', iv: ivBytes }, key, ct)
    return new TextDecoder().decode(plain)
  }

  /**
   * Store a CryptoKey in IndexedDB by name.
   */
  async function storeKey(name: string, key: CryptoKey): Promise<void> {
    const db = await getDb()
    await db.put(STORE_KEYS, key, name)
  }

  /**
   * Retrieve a CryptoKey from IndexedDB by name.
   */
  async function loadKey(name: string): Promise<CryptoKey | null> {
    const db = await getDb()
    return db.get(STORE_KEYS, name) ?? null
  }

  /**
   * Store a full key pair.
   */
  async function storeKeyPair(prefix: string, pair: CryptoKeyPair): Promise<void> {
    await storeKey(`${prefix}:pub`, pair.publicKey)
    await storeKey(`${prefix}:priv`, pair.privateKey)
  }

  /**
   * Load a full key pair.
   */
  async function loadKeyPair(prefix: string): Promise<CryptoKeyPair | null> {
    const pub = await loadKey(`${prefix}:pub`)
    const priv = await loadKey(`${prefix}:priv`)
    if (!pub || !priv) return null
    return { publicKey: pub, privateKey: priv }
  }

  /**
   * Ensure keys exist locally and on the server.
   * Called after login — generates if missing, uploads public keys.
   */
  async function ensureKeys(accessToken: string): Promise<void> {
    const db = await getDb()

    let identityPair = await loadKeyPair('identity')
    let signedPreKeyPair = await loadKeyPair('spk')

    const needsGeneration = !identityPair || !signedPreKeyPair

    if (!identityPair) {
      identityPair = await generateKeyPair()
      await storeKeyPair('identity', identityPair)
    }

    if (!signedPreKeyPair) {
      signedPreKeyPair = await generateKeyPair()
      await storeKeyPair('spk', signedPreKeyPair)
    }

    // Generate batch of one-time prekeys
    const otpkCount = 20
    const otpks: CryptoKeyPair[] = []
    for (let i = 0; i < otpkCount; i++) {
      const pair = await generateKeyPair()
      await storeKeyPair(`otpk:${Date.now()}:${i}`, pair)
      otpks.push(pair)
    }

    if (needsGeneration) {
      // Upload public keys to server
      const identityPubB64 = await exportPublicKey(identityPair.publicKey)
      const spkPubB64 = await exportPublicKey(signedPreKeyPair.publicKey)
      const otpkPubs = await Promise.all(otpks.map(k => exportPublicKey(k.publicKey)))

      // Simple self-signature placeholder (in production use Ed25519)
      const spkSignature = btoa('self-signed:' + spkPubB64.substring(0, 32))

      await fetch(`${config.public.apiBaseUrl}/keys`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${accessToken}`,
        },
        body: JSON.stringify({
          identity_key: identityPubB64,
          signed_prekey: { public_key: spkPubB64, signature: spkSignature },
          one_time_prekeys: otpkPubs,
        }),
      })
    }
  }

  /**
   * Establish a shared key with a peer given their prekey bundle.
   * Returns the derived AES-256-GCM key for message encryption.
   */
  async function establishSession(peerBundle: {
    identity_key: string
    signed_prekey: string
    one_time_prekey: string | null
  }): Promise<CryptoKey> {
    const identityPair = await loadKeyPair('identity')
    if (!identityPair) throw new Error('No identity key found — please log in again')

    // Import peer's signed prekey
    const peerSpk = await importPublicKey(peerBundle.signed_prekey)

    // X3DH simplified: derive shared secret from our identity + peer's spk
    const sharedKey = await deriveSharedKey(identityPair.privateKey, peerSpk)
    return sharedKey
  }

  /**
   * Get or establish session key for a conversation.
   * Session keys are cached in IndexedDB.
   */
  async function getSessionKey(conversationId: string, peerBundle?: {
    identity_key: string
    signed_prekey: string
    one_time_prekey: string | null
  }): Promise<CryptoKey> {
    const cached = await loadKey(`session:${conversationId}`)
    if (cached) return cached

    if (!peerBundle) throw new Error('Need peer bundle to establish new session')

    const key = await establishSession(peerBundle)
    await storeKey(`session:${conversationId}`, key)
    return key
  }

  return {
    generateKeyPair,
    exportPublicKey,
    importPublicKey,
    deriveSharedKey,
    encrypt,
    decrypt,
    storeKey,
    loadKey,
    storeKeyPair,
    loadKeyPair,
    ensureKeys,
    establishSession,
    getSessionKey,
  }
}
