FROM oven/bun

WORKDIR /app

COPY . .

RUN bun install

ENV ENV production
CMD ["./bun", "index.ts"]

EXPOSE 8000