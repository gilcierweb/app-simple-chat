# App Simple Chat

Secure real-time chat platform with end-to-end encryption (E2E), built with a Rust backend and a Nuxt 4 frontend.

## Overview

App Simple Chat is a full-stack messaging application focused on privacy and modern web UX:

- End-to-end encrypted messaging (client-side encryption/decryption)
- Real-time communication via WebSocket
- JWT auth with refresh flow
- User profile and conversation management
- Multi-language frontend (pt-BR, en, es)

## Tech Stack

### Backend
- Rust (Edition 2024)
- Actix Web
- Diesel ORM
- PostgreSQL
- Redis
- Utoipa + Swagger UI

### Frontend
- Nuxt 4
- Vue 3
- Pinia
- Tailwind CSS 4
- FlyonUI 2
- Nuxt i18n
- IndexedDB (`idb`)

## Repository Structure

```text
.
├── backend/        # Rust API (Actix + Diesel)
├── frontend/       # Nuxt 4 app
├── docs/           # Product notes, backlog, design artifacts
├── infra/          # Container-related files (work-in-progress)
└── README.md
```

## Core Features

- Authentication:
  - Register, email confirmation, login, refresh, logout
  - 2FA endpoints available
- Messaging:
  - Create/list conversations
  - Send/list/delete messages
  - Delivery/read receipts
- Encryption:
  - Client key upload (`/keys`)
  - Peer prekey bundle retrieval
  - Web Crypto + local key/session cache
- Realtime:
  - WebSocket room join/leave
  - New message and typing events

## Architecture at a Glance

1. The frontend encrypts message content on the client.
2. The backend stores ciphertext and metadata only.
3. WebSocket broadcasts new message events in conversation rooms.
4. Clients decrypt incoming content using local session keys.

## Prerequisites

- Rust (stable, compatible with edition `2024`)
- Cargo
- Node.js 20+ (recommended)
- pnpm
- PostgreSQL 16+
- Redis 7+

Optional:
- Diesel CLI (for running DB migrations manually)

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Local Development

### 1) Start infrastructure services

Use your own Postgres/Redis, or start them with Docker quickly:

```bash
docker run --name app-chat-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=app_chat -p 5432:5432 -d postgres:16-alpine
docker run --name app-chat-redis -p 6379:6379 -d redis:7-alpine
```

### 2) Configure backend environment

Create `backend/.env` (or export variables in your shell):

```env
HOST=0.0.0.0
PORT=8080
ENVIRONMENT=development
FRONTEND_URL=http://localhost:3000,http://127.0.0.1:3000

DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_chat
REDIS_URL=redis://127.0.0.1:6379

JWT_SECRET=replace_with_a_long_random_secret
DB_POOL_SIZE=10
```

Notes:
- `DATABASE_URL` and `JWT_SECRET` are required.
- `REDIS_URL` has a default, but you should set it explicitly for consistency.

### 3) Run database migrations

From `backend/`:

```bash
diesel migration run
```

### 4) Run backend

From `backend/`:

```bash
cargo run
```

Backend runs by default on:
- `http://localhost:8080`

### 5) Configure frontend environment

Create `frontend/.env`:

```env
NUXT_PUBLIC_API_BASE=http://localhost:8080/api/v1
NUXT_PUBLIC_WS_URL=ws://localhost:8080/api/v1/ws
NUXT_PUBLIC_APP_NAME=Simple Chat
```

### 6) Run frontend

From `frontend/`:

```bash
pnpm install
pnpm dev
```

Frontend runs on:
- `http://localhost:3000`

## Frontend Scripts

From `frontend/`:

```bash
pnpm dev        # start dev server
pnpm build      # production build
pnpm preview    # preview production build
pnpm generate   # static generation
```

## API & Docs

- API base path: `/api/v1`
- Health check: `GET /api/v1/health`
- Swagger UI: `/swagger-ui/`
- OpenAPI JSON: `/api-docs/openapi.json`

### Main endpoint groups

- Auth:
  - `POST /api/v1/auth/register`
  - `GET /api/v1/auth/confirm?token=...`
  - `POST /api/v1/auth/login`
  - `POST /api/v1/auth/refresh`
  - `POST /api/v1/auth/logout`
- Conversations:
  - `GET /api/v1/conversations`
  - `POST /api/v1/conversations`
  - `GET /api/v1/users/lookup?email=...`
- Messages:
  - `GET /api/v1/messages/{conversation_id}`
  - `POST /api/v1/messages/{conversation_id}`
  - `DELETE /api/v1/messages/{conversation_id}/{message_id}`
- Keys:
  - `POST /api/v1/keys`
  - `GET /api/v1/keys/{user_id}`
- WebSocket:
  - `GET /api/v1/ws`
  - `GET /api/v1/ws/token`

## Internationalization

Frontend locales are defined in:

- `frontend/i18n/locales/pt-BR.json`
- `frontend/i18n/locales/en.json`
- `frontend/i18n/locales/es.json`

Nuxt i18n strategy is `prefix_except_default` with `pt-BR` as default locale.

## Security Notes

- Keep `JWT_SECRET` private and rotate regularly.
- Use TLS in production.
- Restrict CORS (`FRONTEND_URL`) to trusted domains.
- Never expose database credentials in client-side code.

## Troubleshooting

- `DATABASE_URL must be set`:
  - Make sure backend env vars are loaded before `cargo run`.
- Frontend cannot connect to API:
  - Verify `NUXT_PUBLIC_API_BASE` and backend port.
- WebSocket connection fails:
  - Check `NUXT_PUBLIC_WS_URL` and CORS/allowed origins.
- Diesel migration issues:
  - Confirm Postgres is up and reachable with your `DATABASE_URL`.

## Current Status

This repository is actively evolving.  
Some files under `docs/` and `infra/` are reference or work-in-progress artifacts and may not match the exact root runtime setup yet.

---

If you want, I can also add:
- a ready-to-use `backend/.env.example`
- a ready-to-use `frontend/.env.example`
- a one-command `Makefile` (`make dev`, `make migrate`, `make up-db`) for onboarding.
