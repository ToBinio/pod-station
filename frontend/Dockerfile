# Stage 1: Build
FROM oven/bun:1.2 AS builder

WORKDIR /app

COPY bun.lock package.json ./
RUN bun install --frozen-lockfile

COPY . .
RUN bun run build

# Stage 2: Run
FROM oven/bun:1.2 AS runner

WORKDIR /app

COPY --from=builder /app/.output .output
COPY --from=builder /app/node_modules node_modules
COPY --from=builder /app/package.json package.json

EXPOSE 3000

CMD ["bun", ".output/server/index.mjs"]
