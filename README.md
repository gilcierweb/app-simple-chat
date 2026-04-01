# App Simple Chat

A full-stack, real-time chat application with end-to-end encryption (E2EE), built with **Rust (Actix + Diesel)** on the backend and **Nuxt 4 (Vue 3)** on the frontend.

## Table of Contents

- [Project Summary](#project-summary)
- [Current Status](#current-status)
- [Tech Stack](#tech-stack)
- [High-Level Architecture](#high-level-architecture)
- [Request and Message Flows](#request-and-message-flows)
- [Chat Crypto Model (Server vs Browser)](#chat-crypto-model-server-vs-browser)
- [Repository Structure (Tree)](#repository-structure-tree)
- [Prerequisites](#prerequisites)
- [Environment Variables](#environment-variables)
- [Run with Docker Compose](#run-with-docker-compose)
- [Run with Docker (Manual Containers)](#run-with-docker-manual-containers)
- [Run Without Docker (Local Native)](#run-without-docker-local-native)
- [Infrastructure Assessment](#infrastructure-assessment)
- [Database Migrations and Seeding](#database-migrations-and-seeding)
- [API and Realtime Endpoints](#api-and-realtime-endpoints)
- [Internationalization (i18n)](#internationalization-i18n)
- [Troubleshooting](#troubleshooting)
- [Security Notes](#security-notes)

## Project Summary

App Simple Chat focuses on secure messaging and a modern UX:

- End-to-end encrypted chat (plaintext handled on client side)
- WebSocket real-time updates (new message, typing, room events)
- JWT auth with refresh flow
- Conversation and user profile management
- Multi-language UI (`pt-BR`, `en`, `es`)

## Current Status

This repository is active and evolving.

Infrastructure has been updated to align with the current project layout:

- `infra/docker-compose.yml` now points to `../backend` and `../frontend` build contexts.
- API and WebSocket routing were aligned in `infra/nginx/nginx.conf` to `/api/v1/*` and `/api/v1/ws`.
- Backend and frontend Dockerfiles are available in their own app folders.

## Tech Stack

### Backend

- [Rust](https://www.rust-lang.org/) (Edition 2024)
- [Actix Web](https://actix.rs/)
- [Diesel ORM](https://diesel.rs/)
- [PostgreSQL](https://www.postgresql.org/)
- [Redis](https://redis.io/)
- [Utoipa](https://github.com/juhaku/utoipa) + [Swagger UI](https://swagger.io/tools/swagger-ui/)

### Frontend

- [Nuxt 4](https://nuxt.com/)
- [Vue 3](https://vuejs.org/)
- [Pinia](https://pinia.vuejs.org/)
- [Tailwind CSS 4](https://tailwindcss.com/)
- [FlyonUI 2](https://flyonui.com/)
- [Nuxt i18n (`@nuxtjs/i18n`)](https://i18n.nuxtjs.org/)
- [IndexedDB](https://developer.mozilla.org/docs/Web/API/IndexedDB_API) via [`idb`](https://github.com/jakearchibald/idb)

## High-Level Architecture

1. Frontend encrypts message content before sending.
2. Backend stores ciphertext + metadata only.
3. Backend emits realtime events through WebSocket rooms.
4. Clients decrypt messages locally using key/session material.

## Request and Message Flows

### Authentication Flow

1. `POST /api/v1/auth/register`
2. User confirms account (`/api/v1/auth/confirm`)
3. `POST /api/v1/auth/login`
4. Client stores access/refresh tokens
5. `POST /api/v1/auth/refresh` when access token expires

### Encrypted Messaging Flow

1. Client uploads key bundle (`POST /api/v1/keys`)
2. Sender fetches recipient bundle (`GET /api/v1/keys/{user_id}`)
3. Sender encrypts message in browser
4. Sender posts ciphertext (`POST /api/v1/messages/{conversation_id}`)
5. Backend broadcasts `new_message` event via WebSocket
6. Receiver decrypts locally

### Conversation Screen Load Flow

1. Frontend resolves conversation by route id
2. Frontend fetches server messages (`GET /api/v1/messages/{conversation_id}`)
3. Frontend attempts local decryption/session resolution
4. Frontend renders decrypted messages and updates local cache

## Chat Crypto Model (Server vs Browser)

### Browser (Frontend) responsibilities

- Generates and stores user key material locally.
- Fetches peer key bundles from `/api/v1/keys/{user_id}`.
- Creates/derives session keys for each peer conversation.
- Encrypts plaintext before sending.
- Decrypts ciphertext after receiving from REST/WebSocket.
- Persists local decrypted cache (for UX/performance), not on server.

### Server (Backend) responsibilities

- Authenticates and authorizes users for conversations.
- Stores only encrypted message payload (`ciphertext`) + metadata (`iv`, `sender_id`, timestamps).
- Delivers encrypted payloads through REST and WebSocket.
- Stores and serves public key bundles (`/keys` endpoints).
- Does not need plaintext to route, persist, or broadcast messages.

### End-to-end message lifecycle

1. User A types a message in browser.
2. Frontend encrypts message with peer/session key.
3. Frontend sends encrypted payload to backend.
4. Backend persists ciphertext in Postgres.
5. Backend broadcasts `new_message` event in conversation room.
6. User B receives encrypted payload and decrypts in browser.
7. UI renders plaintext only on client side.

### Security boundaries

- Backend/database compromise should expose ciphertext, not plaintext history.
- Transport layer still requires HTTPS/WSS in production.
- Local browser storage must be treated as sensitive user data.
- If peer key/session is unavailable, the UI should keep message encrypted or show a safe fallback state.

## Repository Structure (Tree)

```text
.
├── backend
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── diesel.toml
│   ├── migrations
│   └── src
│       ├── main.rs
│       ├── bin/
│       │   └── seed.rs
│       ├── controllers/
│       ├── db/
│       ├── middleware/
│       ├── models/
│       ├── repositories/
│       ├── routes/
│       └── ws/
├── frontend
│   ├── Dockerfile
│   ├── nginx.conf
│   ├── package.json
│   ├── nuxt.config.ts
│   ├── app/
│   │   ├── components/
│   │   ├── composables/
│   │   ├── pages/
│   │   └── stores/
│   └── i18n/
│       └── locales/
├── infra
│   ├── docker-compose.yml
│   ├── Dockerfile
│   └── nginx/
│       └── nginx.conf
└── README.md
```

## Prerequisites

- Rust stable (edition 2024 compatible)
- Cargo
- Node.js 20+ (Node 22 recommended)
- pnpm
- PostgreSQL 16+
- Redis 7+

Optional (for migrations):

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Environment Variables

### Backend (`backend/.env`)

Minimum required variables:

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

### Frontend (`frontend/.env`)

```env
NUXT_PUBLIC_API_BASE=http://localhost:8080/api/v1
NUXT_PUBLIC_WS_URL=ws://localhost:8080/api/v1/ws
NUXT_PUBLIC_APP_NAME=Simple Chat
```

## Run with Docker Compose

Recommended approach:

- Run full stack with Compose (API, frontend, nginx, postgres, redis, minio)
- Or run only infra services and keep API/frontend local

### 1. Create compose environment file

Create `infra/.env` (or export vars in shell):

```bash
POSTGRES_PASSWORD=secret
REDIS_PASSWORD=redissecret
JWT_SECRET=replace_with_long_secret
DB_ENCRYPTION_KEY=replace_with_hex_key
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=miniasecret
FRONTEND_URL=http://localhost:3000
NUXT_PUBLIC_API_BASE=http://localhost/api/v1
NUXT_PUBLIC_WS_URL=ws://localhost/api/v1/ws
```

### 2. Start full stack

```bash
docker compose --env-file infra/.env -f infra/docker-compose.yml up -d --build
```

### 3. Verify services

```bash
docker compose --env-file infra/.env -f infra/docker-compose.yml ps
```

### 4. Run migrations (one time per fresh database)

```bash
cd backend
DATABASE_URL=postgres://simplechat:secret@localhost:5432/simplechat diesel migration run
```

### 5. Open app

- Frontend via nginx: `http://localhost`
- Backend API via nginx: `http://localhost/api/v1`
- WebSocket via nginx: `ws://localhost/api/v1/ws`
- Swagger UI: `http://localhost/swagger-ui/`

### 6. Optional: infra-only mode

Use this when you want backend/frontend running locally with hot reload:

```bash
docker compose --env-file infra/.env -f infra/docker-compose.yml up -d postgres redis
```

## Run with Docker (Manual Containers)

If you prefer not to use compose, run infra containers directly.

### 1. PostgreSQL

```bash
docker run --name app-chat-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=app_chat \
  -p 5432:5432 -d postgres:16-alpine
```

### 2. Redis

```bash
docker run --name app-chat-redis \
  -p 6379:6379 -d redis:7-alpine
```

### 3. Run backend/frontend locally

Follow the same steps in [Run with Docker Compose](#run-with-docker-compose) from env setup onward.

## Run Without Docker (Local Native)

### 1. Install and start PostgreSQL + Redis locally

Use your OS package manager/service manager.

### 2. Create database

Example:

```sql
CREATE DATABASE app_chat;
```

### 3. Configure backend env

```shell
export DATABASE_URL=postgres://postgres:password@localhost:5432/simple_chat_development
openssl rand -base64 32
export JWT_SECRET=your-secret-key-min-32-characters-long
export JWT_EXPIRY_HOURS=8
```

Set `DATABASE_URL` and `REDIS_URL` to your local services.

### 4. Run migrations

```bash
cd backend
diesel migration run
```

### 5. Start backend

```bash
cargo run
```

You can also use:

```bash
cargo run --release
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo fmt --all
```

### 6. Start frontend

```bash
cd frontend
pnpm install
pnpm dev
```

## Infrastructure Assessment

Infra was improved to be closer to a functional end-to-end setup.

### What is already good

- `infra/nginx/nginx.conf` exists and has:
  - upstream blocks for `api` and `frontend`
  - websocket upgrade handling
  - basic security headers
  - request rate-limit zones
- `infra/docker-compose.yml` defines all main services:
  - `postgres`, `redis`, `minio`, `api`, `frontend`, `nginx`, `mailhog`
- Build context paths now match repository layout (`../backend`, `../frontend`)
- API proxy is aligned to backend prefixes (`/api/v1` and `/api/v1/ws`)

### Remaining checks and potential improvements

1. Validate full stack locally after env setup:
   - `docker compose --env-file infra/.env -f infra/docker-compose.yml config`
   - `docker compose --env-file infra/.env -f infra/docker-compose.yml up -d --build`
2. TLS cert mount is optional but still declared:
   - Compose mounts `infra/nginx/certs`; create this directory/files for HTTPS mode.
3. Frontend image serves Nuxt static output:
   - If later you need SSR/runtime server features, switch frontend service to run Nitro server instead of static nginx.

### Recommended improvements

1. Validate full stack locally with:
   - `docker compose -f infra/docker-compose.yml config`
   - `docker compose -f infra/docker-compose.yml up --build`
2. Add an `infra/.env.example` specifically for compose defaults and required secrets.
3. Add CI check to lint compose/nginx config on each PR.

## Database Migrations and Seeding

### Run migrations

```bash
cd backend
diesel migration run
```

### Seed test data

A complete system seed is available at `backend/src/bin/seed.rs`.

```bash
cd backend
cargo run --bin seed
```

Seed includes:

- Roles (`admin`, `operator`, `viewer`)
- 6 test users
- Profiles
- User-role links
- User keys (types 1, 2, 3)
- Sample conversations, members, messages, receipts

## API and Realtime Endpoints

Base path: `/api/v1`

### Core routes

- `GET /api/v1/health`
- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/refresh`
- `POST /api/v1/auth/logout`
- `GET /api/v1/conversations`
- `POST /api/v1/conversations`
- `GET /api/v1/messages/{conversation_id}`
- `POST /api/v1/messages/{conversation_id}`
- `DELETE /api/v1/messages/{conversation_id}/{message_id}`
- `POST /api/v1/keys`
- `GET /api/v1/keys/{user_id}`
- `GET /api/v1/ws`
- `GET /api/v1/ws/token`

### API docs

- Swagger UI: `/swagger-ui/`
- OpenAPI JSON: `/api-docs/openapi.json`

## Internationalization (i18n)

Frontend locales:

- `frontend/i18n/locales/pt-BR.json`
- `frontend/i18n/locales/en.json`
- `frontend/i18n/locales/es.json`

Nuxt i18n strategy:

- `prefix_except_default`
- Default locale: `pt-BR`

## Troubleshooting

- `DATABASE_URL must be set`
  - Ensure `backend/.env` exists and variables are loaded.
- Frontend cannot reach backend
  - Validate `NUXT_PUBLIC_API_BASE` and backend port.
- WebSocket fails
  - Validate `NUXT_PUBLIC_WS_URL` and CORS (`FRONTEND_URL`).
- Migration errors
  - Ensure Postgres is running and reachable from `DATABASE_URL`.
- Compose starts fail for app services (`api`, `frontend`, `nginx`)
  - Use Compose for `postgres`/`redis` only for now, or update infra paths/files.

## Security Notes

- Keep `JWT_SECRET` private.
- Use TLS in production.
- Restrict allowed origins in `FRONTEND_URL`.
- Never store plaintext message content server-side.
- Rotate secrets regularly and avoid committing `.env` files.

https://gilcierweb.com.br