FROM debian:stable-slim

ARG version
ARG TARGETARCH

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update \
    && apt-get install -y curl ffmpeg \
    && case ${TARGETARCH} in arm) ARCH="armhf" ;; arm64) ARCH="arm64hf" ;; 386) ARCH="intel32" ;; amd64) ARCH="amd64" ;; esac \
    && curl http://download.repetier.com/files/server/debian-${ARCH}/Repetier-Server-${version}-Linux.deb -o repetier-server.deb \
    #download.repetier.com has no https
    && dpkg --unpack repetier-server.deb \
    && apt-get -f install -y \
    && rm -rf repetier-server.deb \
    && rm -f /var/lib/dpkg/info/repetier-server.postinst \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /data \
    && sed -i "s/var\/lib\/Repetier-Server/data/g" /usr/local/Repetier-Server/etc/RepetierServer.xml

EXPOSE 3344    

CMD [ "/usr/local/Repetier-Server/bin/RepetierServer", "-c", "/usr/local/Repetier-Server/etc/RepetierServer.xml" ]
