ARG version

FROM caddy:$version-builder-alpine AS builder

RUN xcaddy build \
    --with github.com/caddy-dns/acmedns

FROM caddy:$version-alpine

COPY --from=builder /usr/bin/caddy /usr/bin/caddy
