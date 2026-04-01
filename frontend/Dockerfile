# syntax=docker/dockerfile:1

# -- Stage 1: builder
FROM node:22-alpine AS builder

WORKDIR /app

ARG NUXT_PUBLIC_API_BASE
ARG NUXT_PUBLIC_WS_URL

ENV NUXT_PUBLIC_API_BASE=${NUXT_PUBLIC_API_BASE}
ENV NUXT_PUBLIC_WS_URL=${NUXT_PUBLIC_WS_URL}

COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

# -- Stage 2: runtime (serve static assets with nginx)
FROM nginx:1.27-alpine

COPY --from=builder /app/.output/public /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
