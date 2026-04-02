# syntax=docker/dockerfile:1

# -- Stage 1: builder
FROM node:22-slim AS builder

# install pnpm
RUN npm install -g pnpm

WORKDIR /app

# install dependencies (caching)
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# build app
COPY . .
ARG NUXT_PUBLIC_API_BASE
ARG NUXT_PUBLIC_WS_URL
ENV NUXT_PUBLIC_API_BASE=${NUXT_PUBLIC_API_BASE}
ENV NUXT_PUBLIC_WS_URL=${NUXT_PUBLIC_WS_URL}

RUN pnpm build

# -- Stage 2: runtime (Node.js)
# Nuxt 4 Nitro output runs on Node.js
FROM node:22-slim

WORKDIR /app

# Copy output from builder
COPY --from=builder /app/.output ./.output

# Nuxt/Nitro server environment variables
ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=3000

# Expose Nitro server port
EXPOSE 3000

# Launch Nitro server
CMD ["node", ".output/server/index.mjs"]
