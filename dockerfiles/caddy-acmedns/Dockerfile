ARG version

FROM caddy:$version-builder AS builder

RUN xcaddy build \
    --with github.com/caddy-dns/acmedns

FROM caddy:$version

COPY --from=builder /usr/bin/caddy /usr/bin/caddy
