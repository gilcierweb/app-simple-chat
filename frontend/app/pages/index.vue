<template>
  <main class="min-h-screen bg-dark-950">
    <!-- Navbar -->
    <nav class="fixed top-0 z-50 w-full border-b border-dark-800 bg-dark-950/80 backdrop-blur-md">
      <div class="container mx-auto flex h-16 items-center justify-between px-4">
        <div class="flex items-center gap-2">
          <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-dark-950">
            <span class="icon-[lucide--message-circle] size-6"></span>
          </div>
          <span class="font-display text-xl font-bold text-text-primary">AppSimpleChat</span>
        </div>

        <!-- Desktop Menu -->
        <div class="hidden items-center gap-6 md:flex">
          <a href="#features" class="text-sm text-text-secondary hover:text-primary transition-colors">Recursos</a>
          <a href="#security" class="text-sm text-text-secondary hover:text-primary transition-colors">Segurança</a>
          <a href="#how-it-works" class="text-sm text-text-secondary hover:text-primary transition-colors">Como Funciona</a>
          <a href="#faq" class="text-sm text-text-secondary hover:text-primary transition-colors">FAQ</a>
        </div>

        <div class="flex items-center gap-3">
          <!-- Theme Toggle -->
          <label class="swap swap-rotate hidden md:inline-grid">
            <input type="checkbox" class="theme-controller" value="light" />
            <span class="swap-off icon-[lucide--sun] size-5 text-text-secondary"></span>
            <span class="swap-on icon-[lucide--moon] size-5 text-text-secondary"></span>
          </label>

          <NuxtLink to="/auth/login" class="btn btn-ghost btn-sm hidden md:flex">Entrar</NuxtLink>
          <NuxtLink to="/auth/register" class="btn btn-primary btn-sm">
            <span class="icon-[lucide--rocket] size-4"></span>
            Começar
          </NuxtLink>

          <!-- Mobile Menu Button -->
          <button class="btn btn-ghost btn-sm btn-square md:hidden" onclick="mobileMenuDrawer.showModal()">
            <span class="icon-[lucide--menu] size-5"></span>
          </button>
        </div>
      </div>
    </nav>

    <!-- Mobile Drawer -->
    <dialog id="mobileMenuDrawer" class="drawer drawer-end">
      <div class="drawer-content">
        <button class="btn btn-ghost btn-sm btn-square absolute top-4 right-4" onclick="mobileMenuDrawer.close()">
          <span class="icon-[lucide--x] size-5"></span>
        </button>
        <div class="p-6 space-y-6">
          <!-- Logo -->
          <div class="flex items-center gap-2 pb-4 border-b border-dark-800">
            <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-dark-950">
              <span class="icon-[lucide--message-circle] size-6"></span>
            </div>
            <span class="font-display text-xl font-bold text-text-primary">Menu</span>
          </div>

          <!-- Menu Items -->
          <nav class="menu menu-vertical gap-2">
            <li>
              <a href="#features" onclick="mobileMenuDrawer.close()" class="flex items-center gap-3 text-text-secondary hover:text-primary">
                <span class="icon-[lucide--layers] size-5"></span>
                Recursos
              </a>
            </li>
            <li>
              <a href="#security" onclick="mobileMenuDrawer.close()" class="flex items-center gap-3 text-text-secondary hover:text-primary">
                <span class="icon-[lucide--shield] size-5"></span>
                Segurança
              </a>
            </li>
            <li>
              <a href="#how-it-works" onclick="mobileMenuDrawer.close()" class="flex items-center gap-3 text-text-secondary hover:text-primary">
                <span class="icon-[lucide--workflow] size-5"></span>
                Como Funciona
              </a>
            </li>
            <li>
              <a href="#faq" onclick="mobileMenuDrawer.close()" class="flex items-center gap-3 text-text-secondary hover:text-primary">
                <span class="icon-[lucide--help-circle] size-5"></span>
                FAQ
              </a>
            </li>
          </nav>

          <!-- Actions -->
          <div class="pt-4 border-t border-dark-800 space-y-3">
            <NuxtLink to="/auth/login" class="btn btn-ghost w-full" onclick="mobileMenuDrawer.close()">
              <span class="icon-[lucide--log-in] size-4"></span>
              Entrar
            </NuxtLink>
            <NuxtLink to="/auth/register" class="btn btn-primary w-full" onclick="mobileMenuDrawer.close()">
              <span class="icon-[lucide--rocket] size-4"></span>
              Começar
            </NuxtLink>
          </div>
        </div>
      </div>
      <form method="dialog" class="drawer-backdrop bg-dark-950/80">
        <button>close</button>
      </form>
    </dialog>

    <!-- Section 1: Hero -->
    <section class="relative overflow-hidden pt-32 pb-20 lg:pt-40 lg:pb-32">
      <!-- Background decorative elements -->
      <div class="absolute inset-0 overflow-hidden pointer-events-none">
        <div class="absolute top-20 left-10 w-72 h-72 bg-primary/5 rounded-full blur-3xl"></div>
        <div class="absolute bottom-20 right-10 w-96 h-96 bg-brand-500/5 rounded-full blur-3xl"></div>
      </div>

      <div class="container mx-auto px-4 relative z-10">
        <div class="grid items-center gap-12 lg:grid-cols-2">
          <div class="space-y-8">
            <div class="inline-flex items-center gap-2 rounded-full border border-primary/30 bg-primary/10 px-4 py-1.5">
              <span class="icon-[lucide--shield-check] size-4 text-primary"></span>
              <span class="text-xs font-medium text-primary">Criptografia End-to-End</span>
            </div>
            <h1 class="font-display text-4xl font-bold leading-tight text-text-primary lg:text-6xl">
              Mensagens <span class="text-primary">seguras</span> e privadas
            </h1>
            <p class="text-lg text-text-secondary leading-relaxed">
              App Simple Chat é um aplicativo de mensagens com criptografia end-to-end como o WhatsApp, 
              mas com criptografia adicional no banco de dados para dados sensíveis. Segurança máxima, 
              mesmo que alguém consiga acesso ao banco.
            </p>
            <div class="flex flex-wrap gap-4">
              <NuxtLink to="/auth/register" class="btn btn-primary btn-lg">
                <span class="icon-[lucide--message-circle-plus] size-5"></span>
                Iniciar Chat Agora
              </NuxtLink>
              <button class="btn btn-outline btn-primary btn-lg" onclick="githubModal.showModal()">
                <span class="icon-[lucide--github] size-5"></span>
                Código Fonte
              </button>
            </div>
            <div class="flex items-center gap-4 text-sm text-text-muted">
              <div class="flex -space-x-2 avatar-group">
                <div class="avatar avatar-placeholder w-8 h-8">
                  <div class="bg-brand-600 text-white text-xs">JS</div>
                </div>
                <div class="avatar avatar-placeholder w-8 h-8">
                  <div class="bg-accent-500 text-dark-950 text-xs">MK</div>
                </div>
                <div class="avatar avatar-placeholder w-8 h-8">
                  <div class="bg-primary text-dark-950 text-xs">AL</div>
                </div>
              </div>
              <span>+2.5k usuários ativos</span>
            </div>
          </div>
          <div class="relative">
            <div class="card bg-dark-900 border border-dark-800 shadow-2xl">
              <div class="card-body p-6">
                <div class="space-y-4">
                  <div class="flex items-center justify-between border-b border-dark-800 pb-4">
                    <div class="flex items-center gap-3">
                      <div class="avatar avatar-online">
                        <div class="w-10 rounded-full">
                          <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
                        </div>
                      </div>
                      <div>
                        <p class="font-medium text-text-primary">João Silva</p>
                        <p class="text-xs text-primary">online</p>
                      </div>
                    </div>
                    <div class="flex gap-2">
                      <span class="icon-[lucide--phone] size-5 text-text-muted"></span>
                      <span class="icon-[lucide--video] size-5 text-text-muted"></span>
                    </div>
                  </div>
                  <div class="space-y-3">
                    <div class="chat chat-start">
                      <div class="chat-bubble bg-dark-800 text-text-primary">Oi! Como vai a segurança dos dados?</div>
                    </div>
                    <div class="chat chat-end">
                      <div class="chat-bubble bg-primary text-dark-950">100% criptografada! 🔐</div>
                    </div>
                    <div class="chat chat-start">
                      <div class="chat-bubble bg-dark-800 text-text-primary">Mesmo no banco de dados?</div>
                    </div>
                    <div class="chat chat-end">
                      <div class="chat-bubble bg-primary text-dark-950">Sim, E2E + criptografia no DB! 🛡️</div>
                    </div>
                  </div>
                  <div class="join mt-4 w-full">
                    <input type="text" class="input join-item input-filled flex-1 bg-dark-800" placeholder="Digite sua mensagem..." />
                    <button class="btn btn-primary join-item">
                      <span class="icon-[lucide--send] size-4"></span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
            <div class="absolute -right-4 -top-4 rounded-2xl border border-dark-800 bg-dark-900 p-4 shadow-xl">
              <div class="flex items-center gap-3">
                <span class="icon-[lucide--lock] size-6 text-primary"></span>
                <div>
                  <p class="text-xs text-text-muted">Status</p>
                  <p class="text-sm font-medium text-primary">Seguro</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 2: Features -->
    <section id="features" class="border-t border-dark-800 bg-dark-900 py-20">
      <div class="container mx-auto px-4">
        <div class="mx-auto max-w-3xl text-center">
          <span class="badge badge-primary badge-soft mb-4">Recursos</span>
          <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
            Tudo que você precisa para comunicação segura
          </h2>
          <p class="mt-4 text-text-secondary">
            Funcionalidades completas com foco máximo em segurança e privacidade
          </p>
        </div>
        <div class="mt-12 grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <div class="card bg-dark-950 border border-dark-800 hover:border-primary/50 hover:shadow-lg hover:shadow-primary/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <span class="icon-[lucide--shield] size-6 text-primary"></span>
              </div>
              <h3 class="card-title text-text-primary group-hover:text-primary transition-colors">Criptografia E2E</h3>
              <p class="text-text-secondary">Mensagens criptografadas de ponta a ponta. Somente você e o destinatário podem ler.</p>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-brand-500/50 hover:shadow-lg hover:shadow-brand-500/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-brand-500/10 group-hover:bg-brand-500/20 transition-colors">
                <span class="icon-[lucide--database] size-6 text-brand-500"></span>
              </div>
              <h3 class="card-title text-text-primary group-hover:text-brand-500 transition-colors">DB Criptografado</h3>
              <p class="text-text-secondary">Dados sensíveis criptografados no banco. Seguro mesmo com acesso não autorizado.</p>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-accent-500/50 hover:shadow-lg hover:shadow-accent-500/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-accent-500/10 group-hover:bg-accent-500/20 transition-colors">
                <span class="icon-[lucide--key] size-6 text-accent-500"></span>
              </div>
              <h3 class="card-title text-text-primary group-hover:text-accent-500 transition-colors">Chaves Locais</h3>
              <p class="text-text-secondary">Chaves de criptografia armazenadas localmente no navegador. Você controla seus dados.</p>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-warning/50 hover:shadow-lg hover:shadow-warning/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-warning/10 group-hover:bg-warning/20 transition-colors">
                <span class="icon-[lucide--timer] size-6 text-warning"></span>
              </div>
              <h3 class="card-title text-text-primary group-hover:text-warning transition-colors">Mensagens Temporárias</h3>
              <p class="text-text-secondary">Configure auto-destruição de mensagens. Controle total sobre seu histórico.</p>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-info/50 hover:shadow-lg hover:shadow-info/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-info/10 group-hover:bg-info/20 transition-colors">
                <span class="icon-[lucide--file-image] size-6 text-info"></span>
              </div>
              <h3 class="card-title text-text-primary group-hover:text-info transition-colors">Mídia Segura</h3>
              <p class="text-text-secondary">Fotos, vídeos e documentos com criptografia adicional. Compartilhe com confiança.</p>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-error/50 hover:shadow-lg hover:shadow-error/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-error/10 group-hover:bg-error/20 transition-colors">
                <span class="icon-[lucide--user-x] size-6 text-error"></span>
              </div>
              <h3 class="card-title text-text-primary group-hover:text-error transition-colors">Anti-Replay</h3>
              <p class="text-text-secondary">Proteção contra ataques de replay. Cada mensagem é única e verificável.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 3: Security -->
    <section id="security" class="py-20">
      <div class="container mx-auto px-4">
        <div class="grid items-center gap-12 lg:grid-cols-2">
          <div class="order-2 lg:order-1">
            <div class="space-y-4">
              <div class="alert alert-soft alert-success">
                <span class="icon-[lucide--check-circle] size-5"></span>
                <div>
                  <p class="font-medium">Criptografia AES-256-GCM</p>
                  <p class="text-sm opacity-80">Padrão militar para dados em trânsito</p>
                </div>
              </div>
              <div class="alert alert-soft alert-success">
                <span class="icon-[lucide--check-circle] size-5"></span>
                <div>
                  <p class="font-medium">ECDH Key Exchange</p>
                  <p class="text-sm opacity-80">Curvas elípticas para troca segura de chaves</p>
                </div>
              </div>
              <div class="alert alert-soft alert-success">
                <span class="icon-[lucide--check-circle] size-5"></span>
                <div>
                  <p class="font-medium">Perfect Forward Secrecy</p>
                  <p class="text-sm opacity-80">Chaves temporárias, comprometimento zero</p>
                </div>
              </div>
              <div class="alert alert-soft alert-success">
                <span class="icon-[lucide--check-circle] size-5"></span>
                <div>
                  <p class="font-medium">Web Crypto API</p>
                  <p class="text-sm opacity-80">API nativa do navegador, sem bibliotecas externas</p>
                </div>
              </div>
            </div>
          </div>
          <div class="order-1 lg:order-2">
            <span class="badge badge-primary badge-soft mb-4">Segurança</span>
            <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
              Segurança em camadas
            </h2>
            <p class="mt-4 text-text-secondary leading-relaxed">
              Nossa arquitetura de segurança utiliza múltiplas camadas de proteção. 
              Criptografia end-to-end garante que apenas participantes da conversa leiam as mensagens. 
              Criptografia no banco protege dados sensíveis em repouso.
            </p>
            <div class="mt-6 flex flex-wrap gap-3">
              <div class="tooltip tooltip-top" data-tip="Transport Layer Security 1.3 - Protocolo de criptografia em trânsito">
                <span class="badge badge-outline badge-primary cursor-help">
                  <span class="icon-[lucide--lock] size-3"></span>
                  TLS 1.3
                </span>
              </div>
              <div class="tooltip tooltip-top" data-tip="Secure Hash Algorithm 256-bit - Função hash criptográfica">
                <span class="badge badge-outline badge-primary cursor-help">
                  <span class="icon-[lucide--fingerprint] size-3"></span>
                  SHA-256
                </span>
              </div>
              <div class="tooltip tooltip-top" data-tip="Rivest-Shamir-Adleman 4096-bit - Criptografia assimétrica">
                <span class="badge badge-outline badge-primary cursor-help">
                  <span class="icon-[lucide--key-round] size-3"></span>
                  RSA-4096
                </span>
              </div>
              <div class="tooltip tooltip-top" data-tip="Código de alto desempenho rodando no navegador">
                <span class="badge badge-outline badge-primary cursor-help">
                  <span class="icon-[lucide--cpu] size-3"></span>
                  WebAssembly
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 4: Tech Stack -->
    <section class="border-t border-dark-800 bg-dark-900 py-20">
      <div class="container mx-auto px-4">
        <div class="mx-auto max-w-3xl text-center">
          <span class="badge badge-primary badge-soft mb-4">Tecnologia</span>
          <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
            Stack moderno e performático
          </h2>
          <p class="mt-4 text-text-secondary">
            Construído com as melhores tecnologias para máxima segurança e performance
          </p>
        </div>
        <div class="mt-12 grid gap-8 md:grid-cols-2">
          <div class="card bg-dark-950 border border-dark-800 hover:border-orange-500/50 hover:shadow-lg hover:shadow-orange-500/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex items-center gap-3">
                <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-orange-500/20 group-hover:bg-orange-500/30 transition-colors">
                  <span class="icon-[lucide--server] size-6 text-orange-500"></span>
                </div>
                <div>
                  <h3 class="card-title text-text-primary group-hover:text-orange-500 transition-colors">Backend Rust</h3>
                  <p class="text-sm text-text-muted">Segurança de memória sem GC</p>
                </div>
              </div>
              <ul class="space-y-2 text-text-secondary">
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  Actix Web - Web framework async
                </li>
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  Tokio - Runtime assíncrono
                </li>
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  Diesel - Queries seguras em compile-time
                </li>
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  WebSockets nativos
                </li>
              </ul>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-green-500/50 hover:shadow-lg hover:shadow-green-500/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mb-4 flex items-center gap-3">
                <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-green-500/20 group-hover:bg-green-500/30 transition-colors">
                  <span class="icon-[lucide--layout] size-6 text-green-500"></span>
                </div>
                <div>
                  <h3 class="card-title text-text-primary group-hover:text-green-500 transition-colors">Frontend Nuxt 4</h3>
                  <p class="text-sm text-text-muted">Vue 3 + Nitro SSR/SSG</p>
                </div>
              </div>
              <ul class="space-y-2 text-text-secondary">
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  Vue 3 Composition API
                </li>
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  TailwindCSS 4 + FlyonUI 2
                </li>
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  Pinia - State management
                </li>
                <li class="flex items-center gap-2">
                  <span class="icon-[lucide--check] size-4 text-primary"></span>
                  Web Crypto API nativa
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 5: How It Works -->
    <section id="how-it-works" class="py-20 bg-dark-950">
      <div class="container mx-auto px-4">
        <div class="mx-auto max-w-3xl text-center mb-12">
          <span class="badge badge-primary badge-soft mb-4">Processo</span>
          <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
            Como funciona a criptografia
          </h2>
          <p class="mt-4 text-text-secondary">
            Entenda o fluxo de segurança em 4 passos simples
          </p>
        </div>
        <div class="mt-12 grid gap-6 md:grid-cols-2 lg:grid-cols-4">
          <!-- Step 1 -->
          <div class="card bg-dark-900 border border-dark-800 hover:border-primary/50 hover:shadow-lg hover:shadow-primary/10 transition-all duration-300">
            <div class="card-body items-center text-center">
              <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/20">
                <span class="icon-[lucide--user-plus] size-8 text-primary"></span>
              </div>
              <div class="badge badge-primary badge-soft mb-2">Passo 1</div>
              <h4 class="card-title text-text-primary text-lg">Registro</h4>
              <p class="text-text-secondary text-sm mt-2">Chaves RSA geradas no navegador. Private key nunca sai do dispositivo.</p>
            </div>
          </div>
          <!-- Step 2 -->
          <div class="card bg-dark-900 border border-dark-800 hover:border-primary/50 hover:shadow-lg hover:shadow-primary/10 transition-all duration-300">
            <div class="card-body items-center text-center">
              <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/20">
                <span class="icon-[lucide--key] size-8 text-primary"></span>
              </div>
              <div class="badge badge-primary badge-soft mb-2">Passo 2</div>
              <h4 class="card-title text-text-primary text-lg">Troca de Chaves</h4>
              <p class="text-text-secondary text-sm mt-2">ECDH estabelece chave de sessão única para cada conversa.</p>
            </div>
          </div>
          <!-- Step 3 -->
          <div class="card bg-dark-900 border border-dark-800 hover:border-primary/50 hover:shadow-lg hover:shadow-primary/10 transition-all duration-300">
            <div class="card-body items-center text-center">
              <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/20">
                <span class="icon-[lucide--message-square] size-8 text-primary"></span>
              </div>
              <div class="badge badge-primary badge-soft mb-2">Passo 3</div>
              <h4 class="card-title text-text-primary text-lg">Envio Seguro</h4>
              <p class="text-text-secondary text-sm mt-2">Mensagem criptografada com AES-256-GCM antes de sair do dispositivo.</p>
            </div>
          </div>
          <!-- Step 4 -->
          <div class="card bg-dark-900 border border-dark-800 hover:border-primary/50 hover:shadow-lg hover:shadow-primary/10 transition-all duration-300">
            <div class="card-body items-center text-center">
              <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/20">
                <span class="icon-[lucide--unlock] size-8 text-primary"></span>
              </div>
              <div class="badge badge-primary badge-soft mb-2">Passo 4</div>
              <h4 class="card-title text-text-primary text-lg">Descriptografia</h4>
              <p class="text-text-secondary text-sm mt-2">Apenas o destinatário com a chave privada pode ler a mensagem.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 6: Privacy -->
    <section class="border-t border-dark-800 bg-dark-900 py-20">
      <div class="container mx-auto px-4">
        <div class="grid items-center gap-12 lg:grid-cols-2">
          <div>
            <span class="badge badge-primary badge-soft mb-4">Privacidade</span>
            <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
              Seus dados são seus
            </h2>
            <p class="mt-4 text-text-secondary leading-relaxed">
              Não coletamos metadados. Não vendemos dados. Não temos acesso às suas conversas. 
              Nossa arquitetura é projetada para privacidade por padrão.
            </p>
            <div class="mt-6 space-y-3">
              <div class="flex items-center gap-3">
                <span class="icon-[lucide--eye-off] size-5 text-primary"></span>
                <span class="text-text-secondary">Sem rastreamento de metadados</span>
              </div>
              <div class="flex items-center gap-3">
                <span class="icon-[lucide--trash-2] size-5 text-primary"></span>
                <span class="text-text-secondary">Auto-deleção configurável</span>
              </div>
              <div class="flex items-center gap-3">
                <span class="icon-[lucide--hard-drive] size-5 text-primary"></span>
                <span class="text-text-secondary">Dados criptografados no dispositivo</span>
              </div>
              <div class="flex items-center gap-3">
                <span class="icon-[lucide--file-x] size-5 text-primary"></span>
                <span class="text-text-secondary">Sem backups não criptografados</span>
              </div>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800">
            <div class="card-body">
              <h4 class="card-title text-text-primary mb-4">
                <span class="icon-[lucide--database-zap] size-5 text-primary"></span>
                Criptografia no Banco de Dados
              </h4>
              <div class="space-y-3 text-sm">
                <div class="rounded-lg bg-dark-800 p-3">
                  <p class="text-text-muted">Mensagem em texto puro</p>
                  <p class="text-error line-through">"Olá, como vai?"</p>
                </div>
                <div class="flex justify-center">
                  <span class="icon-[lucide--arrow-down] size-5 text-primary"></span>
                </div>
                <div class="rounded-lg bg-dark-800 p-3">
                  <p class="text-text-muted">Armazenado no banco (criptografado)</p>
                  <p class="text-primary font-mono text-xs break-all">U2FsdGVkX1+7J8v2...9mK3pL8nQ5rT7w==</p>
                </div>
                <div class="alert alert-soft alert-info mt-4">
                  <span class="icon-[lucide--info] size-4"></span>
                  <span class="text-sm">Mesmo com acesso ao DB, os dados permanecem ilegíveis</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 7: Download/App -->
    <section class="py-20">
      <div class="container mx-auto px-4">
        <div class="mx-auto max-w-3xl text-center">
          <span class="badge badge-primary badge-soft mb-4">Acesso</span>
          <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
            Comece a usar agora
          </h2>
          <p class="mt-4 text-text-secondary">
            Acesse diretamente pelo navegador. Não requer instalação.
          </p>
        </div>
        <div class="mt-12 grid gap-6 md:grid-cols-3">
          <div class="card bg-dark-900 border border-dark-800 text-center hover:border-primary/50 hover:shadow-lg hover:shadow-primary/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <span class="icon-[lucide--globe] size-8 text-primary"></span>
              </div>
              <h3 class="card-title justify-center text-text-primary group-hover:text-primary transition-colors">Web App</h3>
              <p class="text-text-secondary text-sm">Acesse de qualquer navegador moderno</p>
              <NuxtLink to="/auth/register" class="btn btn-primary btn-soft mt-4 w-full">
                <span class="icon-[lucide--external-link] size-4"></span>
                Abrir App
              </NuxtLink>
            </div>
          </div>
          <div class="card bg-dark-900 border border-dark-800 text-center hover:border-brand-500/50 hover:shadow-lg hover:shadow-brand-500/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-brand-500/10 group-hover:bg-brand-500/20 transition-colors">
                <span class="icon-[lucide--smartphone] size-8 text-brand-500"></span>
              </div>
              <h3 class="card-title justify-center text-text-primary group-hover:text-brand-500 transition-colors">PWA</h3>
              <p class="text-text-secondary text-sm">Instale como app no celular</p>
              <button class="btn btn-primary btn-soft mt-4 w-full" onclick="pwaModal.showModal()">
                <span class="icon-[lucide--download] size-4"></span>
                Instalar
              </button>
            </div>
          </div>
          <div class="card bg-dark-900 border border-dark-800 text-center hover:border-accent-500/50 hover:shadow-lg hover:shadow-accent-500/10 transition-all duration-300 group">
            <div class="card-body">
              <div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-accent-500/10 group-hover:bg-accent-500/20 transition-colors">
                <span class="icon-[lucide--code-2] size-8 text-accent-500"></span>
              </div>
              <h3 class="card-title justify-center text-text-primary group-hover:text-accent-500 transition-colors">Self-Host</h3>
              <p class="text-text-secondary text-sm">Hospede seu próprio servidor</p>
              <button class="btn btn-primary btn-soft mt-4 w-full" onclick="githubModal.showModal()">
                <span class="icon-[lucide--github] size-4"></span>
                GitHub
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 8: Testimonials -->
    <section class="border-t border-dark-800 bg-dark-900 py-20">
      <div class="container mx-auto px-4">
        <div class="mx-auto max-w-3xl text-center">
          <span class="badge badge-primary badge-soft mb-4">Depoimentos</span>
          <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
            O que dizem sobre nós
          </h2>
        </div>
        <div class="mt-12 grid gap-6 md:grid-cols-3">
          <div class="card bg-dark-950 border border-dark-800 hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5 transition-all duration-300">
            <div class="card-body">
              <div class="flex gap-1 mb-4">
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
              </div>
              <p class="text-text-secondary mb-4 italic">"Finalmente um chat que entende de verdade sobre segurança. A criptografia no banco é um diferencial enorme."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-10 rounded-full">
                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Ana" alt="Ana" />
                  </div>
                </div>
                <div>
                  <p class="font-medium text-text-primary">Ana Costa</p>
                  <p class="text-xs text-text-muted">CISO, TechCorp</p>
                </div>
              </div>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5 transition-all duration-300">
            <div class="card-body">
              <div class="flex gap-1 mb-4">
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
              </div>
              <p class="text-text-secondary mb-4 italic">"Uso para comunicação interna da equipe. A performance do Rust + segurança da criptografia E2E é perfeita."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-10 rounded-full">
                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Pedro" alt="Pedro" />
                  </div>
                </div>
                <div>
                  <p class="font-medium text-text-primary">Pedro Lima</p>
                  <p class="text-xs text-text-muted">CTO, StartupXYZ</p>
                </div>
              </div>
            </div>
          </div>
          <div class="card bg-dark-950 border border-dark-800 hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5 transition-all duration-300">
            <div class="card-body">
              <div class="flex gap-1 mb-4">
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
                <span class="icon-[lucide--star] size-4 text-warning fill-warning"></span>
              </div>
              <p class="text-text-secondary mb-4 italic">"O código aberto me deu confiança. Pude auditar a implementação da criptografia e tudo está impecável."</p>
              <div class="flex items-center gap-3">
                <div class="avatar">
                  <div class="w-10 rounded-full">
                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Marina" alt="Marina" />
                  </div>
                </div>
                <div>
                  <p class="font-medium text-text-primary">Marina Souza</p>
                  <p class="text-xs text-text-muted">Pentester</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 9: FAQ -->
    <section id="faq" class="py-20">
      <div class="container mx-auto px-4">
        <div class="mx-auto max-w-3xl">
          <div class="text-center mb-12">
            <span class="badge badge-primary badge-soft mb-4">FAQ</span>
            <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
              Perguntas frequentes
            </h2>
          </div>

          <!-- Accordion FlyonUI 2 -->
          <div class="accordion accordion-shadow *:accordion-item-active:shadow-md">
            <!-- Item 1 -->
            <div class="accordion-item accordion-item-active:scale-[1.02] accordion-item-active:mb-3 active transition-transform ease-in duration-300 delay-[1ms] bg-dark-900 border border-dark-800 rounded-lg" id="faq-e2e">
              <button class="accordion-toggle inline-flex items-center gap-x-4 px-5 py-4 text-start w-full" aria-controls="faq-e2e-collapse" aria-expanded="true">
                <span class="icon-[lucide--plus] accordion-item-active:hidden text-primary size-5 block shrink-0"></span>
                <span class="icon-[lucide--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
                <span class="font-medium text-text-primary flex items-center gap-3">
                  <span class="icon-[lucide--shield-question] size-5 text-primary"></span>
                  Como a criptografia end-to-end funciona?
                </span>
              </button>
              <div id="faq-e2e-collapse" class="accordion-content w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-e2e" role="region">
                <div class="px-5 pb-4">
                  <p class="text-text-secondary pl-8">
                    As mensagens são criptografadas no seu dispositivo antes de serem enviadas. Apenas o destinatário possui a chave privada necessária para descriptografar. Nossos servidores processam apenas dados cifrados, sem acesso ao conteúdo.
                  </p>
                </div>
              </div>
            </div>

            <!-- Item 2 -->
            <div class="accordion-item accordion-item-active:scale-[1.02] accordion-item-active:my-3 transition-transform ease-in duration-300 delay-[1ms] bg-dark-900 border border-dark-800 rounded-lg" id="faq-db">
              <button class="accordion-toggle inline-flex items-center gap-x-4 px-5 py-4 text-start w-full" aria-controls="faq-db-collapse" aria-expanded="false">
                <span class="icon-[lucide--plus] accordion-item-active:hidden text-primary size-5 block shrink-0"></span>
                <span class="icon-[lucide--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
                <span class="font-medium text-text-primary flex items-center gap-3">
                  <span class="icon-[lucide--database-lock] size-5 text-primary"></span>
                  O que acontece se o banco de dados for comprometido?
                </span>
              </button>
              <div id="faq-db-collapse" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-db" role="region">
                <div class="px-5 pb-4">
                  <p class="text-text-secondary pl-8">
                    Todos os dados sensíveis são criptografados antes de serem armazenados. Mesmo com acesso completo ao banco, um invasor só veria dados cifrados ilegíveis. Cada conversa tem chaves únicas, tornando a quebra praticamente impossível.
                  </p>
                </div>
              </div>
            </div>

            <!-- Item 3 -->
            <div class="accordion-item accordion-item-active:scale-[1.02] accordion-item-active:my-3 transition-transform ease-in duration-300 delay-[1ms] bg-dark-900 border border-dark-800 rounded-lg" id="faq-keys">
              <button class="accordion-toggle inline-flex items-center gap-x-4 px-5 py-4 text-start w-full" aria-controls="faq-keys-collapse" aria-expanded="false">
                <span class="icon-[lucide--plus] accordion-item-active:hidden text-primary size-5 block shrink-0"></span>
                <span class="icon-[lucide--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
                <span class="font-medium text-text-primary flex items-center gap-3">
                  <span class="icon-[lucide--key-round] size-5 text-primary"></span>
                  As chaves de criptografia ficam onde?
                </span>
              </button>
              <div id="faq-keys-collapse" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-keys" role="region">
                <div class="px-5 pb-4">
                  <p class="text-text-secondary pl-8">
                    As chaves privadas são geradas e armazenadas localmente no seu navegador usando IndexedDB com criptografia adicional. Nunca transmitimos chaves privadas para o servidor. Você pode exportar suas chaves para backup.
                  </p>
                </div>
              </div>
            </div>

            <!-- Item 4 -->
            <div class="accordion-item accordion-item-active:scale-[1.02] accordion-item-active:my-3 transition-transform ease-in duration-300 delay-[1ms] bg-dark-900 border border-dark-800 rounded-lg" id="faq-opensource">
              <button class="accordion-toggle inline-flex items-center gap-x-4 px-5 py-4 text-start w-full" aria-controls="faq-opensource-collapse" aria-expanded="false">
                <span class="icon-[lucide--plus] accordion-item-active:hidden text-primary size-5 block shrink-0"></span>
                <span class="icon-[lucide--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
                <span class="font-medium text-text-primary flex items-center gap-3">
                  <span class="icon-[lucide--code-2] size-5 text-primary"></span>
                  É código aberto?
                </span>
              </button>
              <div id="faq-opensource-collapse" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-opensource" role="region">
                <div class="px-5 pb-4">
                  <p class="text-text-secondary pl-8">
                    Sim! Todo o código está disponível no GitHub sob licença MIT. Isso permite auditorias de segurança pela comunidade e garante transparência sobre como suas mensagens são processadas.
                  </p>
                </div>
              </div>
            </div>

            <!-- Item 5 -->
            <div class="accordion-item accordion-item-active:scale-[1.02] accordion-item-active:my-3 transition-transform ease-in duration-300 delay-[1ms] bg-dark-900 border border-dark-800 rounded-lg" id="faq-selfhost">
              <button class="accordion-toggle inline-flex items-center gap-x-4 px-5 py-4 text-start w-full" aria-controls="faq-selfhost-collapse" aria-expanded="false">
                <span class="icon-[lucide--plus] accordion-item-active:hidden text-primary size-5 block shrink-0"></span>
                <span class="icon-[lucide--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
                <span class="font-medium text-text-primary flex items-center gap-3">
                  <span class="icon-[lucide--server-cog] size-5 text-primary"></span>
                  Posso hospedar meu próprio servidor?
                </span>
              </button>
              <div id="faq-selfhost-collapse" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-selfhost" role="region">
                <div class="px-5 pb-4">
                  <p class="text-text-secondary pl-8">
                    Sim! Fornecemos Docker Compose e instruções detalhadas para self-hosting. Você pode rodar em sua própria infraestrutura mantendo todas as funcionalidades de segurança.
                  </p>
                </div>
              </div>
            </div>

            <!-- Item 6 -->
            <div class="accordion-item accordion-item-active:scale-[1.02] accordion-item-active:mt-3 transition-transform ease-in duration-300 delay-[1ms] bg-dark-900 border border-dark-800 rounded-lg" id="faq-limits">
              <button class="accordion-toggle inline-flex items-center gap-x-4 px-5 py-4 text-start w-full" aria-controls="faq-limits-collapse" aria-expanded="false">
                <span class="icon-[lucide--plus] accordion-item-active:hidden text-primary size-5 block shrink-0"></span>
                <span class="icon-[lucide--minus] accordion-item-active:block text-primary size-5 hidden shrink-0"></span>
                <span class="font-medium text-text-primary flex items-center gap-3">
                  <span class="icon-[lucide--gauge] size-5 text-primary"></span>
                  Há limite de mensagens ou usuários?
                </span>
              </button>
              <div id="faq-limits-collapse" class="accordion-content hidden w-full overflow-hidden transition-[height] duration-300" aria-labelledby="faq-limits" role="region">
                <div class="px-5 pb-4">
                  <p class="text-text-secondary pl-8">
                    Não há limites artificiais. A capacidade depende apenas dos recursos do servidor. Criptografia pode adicionar pequena latência, mas mantém performance excelente mesmo com milhares de usuários ativos.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 10: CTA -->
    <section class="border-t border-dark-800 bg-dark-900 py-20">
      <div class="container mx-auto px-4">
        <div class="card bg-gradient-to-br from-primary/20 to-brand-600/20 border border-primary/30">
          <div class="card-body items-center text-center py-16">
            <div class="flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 mb-6">
              <span class="icon-[lucide--message-circle-heart] size-10 text-primary"></span>
            </div>
            <h2 class="font-display text-3xl font-bold text-text-primary lg:text-4xl">
              Pronto para conversas seguras?
            </h2>
            <p class="mt-4 max-w-xl text-text-secondary">
              Junte-se a milhares de usuários que escolheram privacidade real. 
              Sem compromissos, sem custos, 100% open source.
            </p>
            <div class="mt-8 flex flex-wrap justify-center gap-4">
              <NuxtLink to="/auth/register" class="btn btn-primary btn-lg">
                <span class="icon-[lucide--rocket] size-5"></span>
                Começar Gratuitamente
              </NuxtLink>
              <button class="btn btn-outline btn-primary btn-lg" onclick="docsModal.showModal()">
                <span class="icon-[lucide--book-open] size-5"></span>
                Documentação
              </button>
            </div>
            <p class="mt-4 text-sm text-text-muted">
              <span class="icon-[lucide--github] size-4 inline"></span>
              Código aberto em Rust + Nuxt 4
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Section 11: Footer -->
    <footer class="border-t border-dark-800 bg-dark-950 py-12">
      <div class="container mx-auto px-4">
        <div class="grid gap-8 md:grid-cols-4">
          <div class="md:col-span-2">
            <div class="flex items-center gap-2 mb-4">
              <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-dark-950">
                <span class="icon-[lucide--message-circle] size-6"></span>
              </div>
              <span class="font-display text-xl font-bold text-text-primary">AppSimpleChat</span>
            </div>
            <p class="text-text-secondary max-w-sm">
              Chat seguro com criptografia end-to-end e criptografia no banco de dados. 
              Privacidade real, código aberto.
            </p>
            <div class="flex gap-4 mt-4">
              <a href="#" class="text-text-muted hover:text-primary transition-colors">
                <span class="icon-[lucide--github] size-5"></span>
              </a>
              <a href="#" class="text-text-muted hover:text-primary transition-colors">
                <span class="icon-[lucide--twitter] size-5"></span>
              </a>
              <a href="#" class="text-text-muted hover:text-primary transition-colors">
                <span class="icon-[lucide--mail] size-5"></span>
              </a>
            </div>
          </div>
          <div>
            <h4 class="font-medium text-text-primary mb-4">Produto</h4>
            <ul class="space-y-2 text-sm">
              <li><a href="#features" class="text-text-secondary hover:text-primary transition-colors">Recursos</a></li>
              <li><a href="#security" class="text-text-secondary hover:text-primary transition-colors">Segurança</a></li>
              <li><a href="#" class="text-text-secondary hover:text-primary transition-colors">Roadmap</a></li>
              <li><a href="#" class="text-text-secondary hover:text-primary transition-colors">Changelog</a></li>
            </ul>
          </div>
          <div>
            <h4 class="font-medium text-text-primary mb-4">Desenvolvedores</h4>
            <ul class="space-y-2 text-sm">
              <li><a href="#" class="text-text-secondary hover:text-primary transition-colors">Documentação</a></li>
              <li><a href="#" class="text-text-secondary hover:text-primary transition-colors">API</a></li>
              <li><a href="#" class="text-text-secondary hover:text-primary transition-colors">GitHub</a></li>
              <li><a href="#" class="text-text-secondary hover:text-primary transition-colors">Self-Host</a></li>
            </ul>
          </div>
        </div>
        <div class="mt-12 pt-8 border-t border-dark-800 flex flex-col md:flex-row justify-between items-center gap-4">
          <p class="text-sm text-text-muted">
            © 2026 AppSimpleChat. Open source sob MIT License.
          </p>
          <div class="flex gap-6 text-sm">
            <a href="#" class="text-text-muted hover:text-primary transition-colors">Privacidade</a>
            <a href="#" class="text-text-muted hover:text-primary transition-colors">Termos</a>
            <a href="#" class="text-text-muted hover:text-primary transition-colors">Segurança</a>
          </div>
        </div>
      </div>
    </footer>
  </main>

  <!-- Modals -->
  <!-- PWA Modal -->
  <dialog id="pwaModal" class="modal">
    <div class="modal-box bg-dark-900 border border-dark-800 max-w-md">
      <div class="flex items-center gap-3 mb-4">
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-brand-500/20">
          <span class="icon-[lucide--smartphone] size-6 text-brand-500"></span>
        </div>
        <div>
          <h3 class="font-display text-xl font-bold text-text-primary">Instalar PWA</h3>
          <p class="text-sm text-text-muted">Progressive Web App</p>
        </div>
      </div>
      <div class="space-y-4 text-text-secondary">
        <p>Para instalar o AppSimpleChat no seu dispositivo:</p>
        <ol class="list-decimal list-inside space-y-2 text-sm">
          <li>Abra o app no navegador Chrome/Edge/Safari</li>
          <li>Clique no ícone de instalação na barra de endereço</li>
          <li>Confirme a instalação</li>
          <li>O app aparecerá na home screen</li>
        </ol>
        <div class="alert alert-soft alert-info mt-4">
          <span class="icon-[lucide--info] size-4"></span>
          <span class="text-sm">Funciona offline e recebe notificações push!</span>
        </div>
      </div>
      <div class="modal-action mt-6">
        <button class="btn btn-primary" onclick="pwaModal.close()">Entendi</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop bg-dark-950/80">
      <button>close</button>
    </form>
  </dialog>

  <!-- GitHub Modal -->
  <dialog id="githubModal" class="modal">
    <div class="modal-box bg-dark-900 border border-dark-800 max-w-md">
      <div class="flex items-center gap-3 mb-4">
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-accent-500/20">
          <span class="icon-[lucide--github] size-6 text-accent-500"></span>
        </div>
        <div>
          <h3 class="font-display text-xl font-bold text-text-primary">Código Aberto</h3>
          <p class="text-sm text-text-muted">Self-hosting disponível</p>
        </div>
      </div>
      <div class="space-y-4 text-text-secondary">
        <p>O AppSimpleChat é 100% open source sob licença MIT.</p>
        <div class="space-y-3 text-sm">
          <div class="flex items-center gap-3">
            <span class="icon-[lucide--check-circle] size-5 text-primary"></span>
            <span>Código fonte completo no GitHub</span>
          </div>
          <div class="flex items-center gap-3">
            <span class="icon-[lucide--check-circle] size-5 text-primary"></span>
            <span>Docker Compose incluído</span>
          </div>
          <div class="flex items-center gap-3">
            <span class="icon-[lucide--check-circle] size-5 text-primary"></span>
            <span>Documentação de self-hosting</span>
          </div>
          <div class="flex items-center gap-3">
            <span class="icon-[lucide--check-circle] size-5 text-primary"></span>
            <span>Contribuições bem-vindas!</span>
          </div>
        </div>
      </div>
      <div class="modal-action mt-6 gap-2">
        <button class="btn btn-ghost" onclick="githubModal.close()">Fechar</button>
        <a href="https://github.com" target="_blank" class="btn btn-primary">
          <span class="icon-[lucide--external-link] size-4"></span>
          Ver GitHub
        </a>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop bg-dark-950/80">
      <button>close</button>
    </form>
  </dialog>

  <!-- Docs Modal -->
  <dialog id="docsModal" class="modal">
    <div class="modal-box bg-dark-900 border border-dark-800 max-w-md">
      <div class="flex items-center gap-3 mb-4">
        <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/20">
          <span class="icon-[lucide--book-open] size-6 text-primary"></span>
        </div>
        <div>
          <h3 class="font-display text-xl font-bold text-text-primary">Documentação</h3>
          <p class="text-sm text-text-muted">Guia completo do app</p>
        </div>
      </div>
      <div class="space-y-3 text-text-secondary text-sm">
        <div class="menu menu-vertical gap-1">
          <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-dark-800 transition-colors">
            <span class="icon-[lucide--rocket] size-4 text-primary"></span>
            <span>Guia de Início Rápido</span>
          </a>
          <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-dark-800 transition-colors">
            <span class="icon-[lucide--shield] size-4 text-primary"></span>
            <span>Arquitetura de Segurança</span>
          </a>
          <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-dark-800 transition-colors">
            <span class="icon-[lucide--key] size-4 text-primary"></span>
            <span>Gerenciamento de Chaves</span>
          </a>
          <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-dark-800 transition-colors">
            <span class="icon-[lucide--server] size-4 text-primary"></span>
            <span>Self-Hosting Guide</span>
          </a>
          <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-dark-800 transition-colors">
            <span class="icon-[lucide--code-2] size-4 text-primary"></span>
            <span>API Reference</span>
          </a>
        </div>
      </div>
      <div class="modal-action mt-6">
        <button class="btn btn-primary" onclick="docsModal.close()">Fechar</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop bg-dark-950/80">
      <button>close</button>
    </form>
  </dialog>
</template>

<script setup>
definePageMeta({
  layout: 'landing'
})
useHead({
  title: 'AppSimpleChat - Chat Seguro com Criptografia End-to-End',
  meta: [
    { name: 'description', content: 'Chat simples com criptografia end-to-end como WhatsApp, mas com criptografia adicional no banco para dados sensíveis. Backend Rust, frontend Nuxt 4.' }
  ]
})
</script>