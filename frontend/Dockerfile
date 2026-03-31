# ─── Stage 1: builder ─────────────────────────────────────────────────────────
FROM node:22-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

# ─── Stage 2: runtime (serve with nginx) ──────────────────────────────────────
FROM nginx:1.27-alpine

COPY --from=builder /app/.output/public /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
