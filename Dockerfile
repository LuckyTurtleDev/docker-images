#
# Dockerfile for youtube-dl
#

FROM alpine
MAINTAINER kev <noreply@easypi.pro>

RUN set -xe \
    && echo "@edge https://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories \
    && apk add --no-cache ca-certificates \
                          ffmpeg \
                          openssl \
                          aria2 \
                          youtube-dl \
                          atomicparsley@edge \
                          procps #needed for runonce script

# Try to run it so we know it works
RUN youtube-dl --version

WORKDIR /data

COPY --chown=root:root entrypoint.sh /
COPY --chown=root:root runonce.sh /bin/runonce

RUN chmod +x /entrypoint.sh \
 && chmod +x /bin/runonce \
 && echo "* * * * * echo please mount cron tasks to /tasks.cron" >> /tasks.cron

RUN addgroup -g 1000 -S dockeruser \
 && adduser -h /data -u 1000 -S dockeruser -G dockeruser

ENTRYPOINT  ["/entrypoint.sh"]
