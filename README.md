# App Simple Chat

A full-stack, real-time chat application with end-to-end encryption (E2EE), built with **Rust (Actix + Diesel)** on the backend and **Nuxt 4 (Vue 3)** on the frontend.

## Table of Contents

- [Project Summary](#project-summary)
- [Current Status](#current-status)
- [Tech Stack](#tech-stack)
- [High-Level Architecture](#high-level-architecture)
- [Request and Message Flows](#request-and-message-flows)
- [Repository Structure (Tree)](#repository-structure-tree)
- [Prerequisites](#prerequisites)
- [Environment Variables](#environment-variables)
- [Run with Docker Compose](#run-with-docker-compose)
- [Run with Docker (Manual Containers)](#run-with-docker-manual-containers)
- [Run Without Docker (Local Native)](#run-without-docker-local-native)
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

Important note about infrastructure:

- There is a compose file at `infra/docker-compose.yml`, but parts of it reference files/paths that are not fully present in the root runtime layout (for example backend build context and nginx config paths).
- For a reliable developer setup today, use:
  - Docker Compose for infrastructure services only (`postgres`, `redis`) plus local backend/frontend.
  - Or run everything locally without Docker.

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

## Repository Structure (Tree)

```text
.
├── backend
│   ├── Cargo.toml
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
│   └── Dockerfile
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

Recommended approach with current repository state:

- Use Compose to run infra services (`postgres`, `redis`)
- Run backend/frontend locally

### 1. Start infra services

From repository root:

```bash
docker compose -f infra/docker-compose.yml up -d postgres redis
```

### 2. Verify services

```bash
docker compose -f infra/docker-compose.yml ps
```

### 3. Configure backend/frontend env files

Create `backend/.env` and `frontend/.env` using the examples above.

### 4. Run migrations

```bash
cd backend
diesel migration run
```

### 5. Run backend

```bash
cargo run
```

Backend command options (inside `backend/`):

```bash
# Fast compile check (no binary generated)
cargo check

# Debug run (default for development)
cargo run

# Optimized release run
cargo run --release

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release

# Run tests
cargo test

# Lint suggestions
cargo clippy --all-targets --all-features

# Format code
cargo fmt --all
```

### 6. Run frontend

In another terminal:

```bash
cd frontend
pnpm install
pnpm dev
```

### 7. Open app

- Frontend: `http://localhost:3000`
- Backend API: `http://localhost:8080/api/v1`
- Swagger UI: `http://localhost:8080/swagger-ui/`

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
