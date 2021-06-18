#
# Dockerfile for youtube-dl
#

FROM alpine:edge
MAINTAINER kev <noreply@easypi.pro>

RUN set -xe \
    && apk add --no-cache ca-certificates \
                          ffmpeg \
                          openssl \
                          aria2 \
                          youtube-dl \
                          atomicparsley

# Try to run it so we know it works
RUN youtube-dl --version

WORKDIR /data

COPY entrypoint.sh /

RUN chmod +x /entrypoint.sh \
 && echo "* * * * * echo please mount cron tasks to /tasks.cron" >> /tasks.cron

RUN addgroup -g 1000 -S dockeruser \
 && adduser -h /data -u 1000 -S dockeruser -G dockeruser

ENTRYPOINT  ["/entrypoint.sh"]
