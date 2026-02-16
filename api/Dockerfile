FROM node:24-alpine AS builder

WORKDIR /app

RUN corepack enable

COPY package*.json .
RUN npm ci
COPY . .

RUN npm run build
RUN npm prune --production

FROM node:24-alpine

WORKDIR /app

RUN apk --no-cache add curl

COPY --from=builder /app/data/ data/
COPY --from=builder /app/dist/ dist/
COPY --from=builder /app/node_modules/ node_modules/

ENV NODE_ENV=production
ENV PORT=3000
ENV HOST=0.0.0.0

EXPOSE 3000

CMD ["node", "/app/dist"]

HEALTHCHECK \
  --interval=1m \
  --timeout=10s \
  --start-period=5s \
  --retries=10 \
  CMD curl -f http://localhost:3000/health || exit 1
