#
# Dockerfile for youtube-dl
#

FROM alpine

RUN set -xe \
    && echo "@edge https://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories \
    && apk add --no-cache ca-certificates \
                          openssl \
                          yt-dlp \
                          aria2 \
                          atomicparsley@edge \
                          attr \
                          ffmpeg \
                          py3-brotli\
                          py3-mutagen\
                          py3-pycryptodomex\
                          py3-secretstorage\
                          py3-websockets \
                          rtmpdump
# see https://archlinux.org/packages/extra/any/yt-dlp/ for opt yt-dlp deps

# Try to run it so we know it works
RUN yt-dlp --version

WORKDIR /data

RUN addgroup -g 1000 -S dockeruser \
 && adduser -h /data -u 1000 -S dockeruser -G dockeruser
