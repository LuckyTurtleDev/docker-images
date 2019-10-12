FROM debian:buster-slim

LABEL maintainer="felix.yadomi@gmail.com"
LABEL version="v0.92.0"

ADD http://download.repetier.com/files/server/debian-armhf/Repetier-Server-0.92.3-Linux.deb repetier-server.deb

RUN apt-get update \
    && dpkg --unpack repetier-server.deb \
    && apt-get install -y avrdude \
    && rm -rf repetier-server.deb \
    && rm -f /var/lib/dpkg/info/repetier-server.postinst

RUN mkdir -p /data \
    && sed -i "s/var\/lib\/Repetier-Server/data/g" /usr/local/Repetier-Server/etc/RepetierServer.xml

EXPOSE 3344    

CMD [ "/usr/local/Repetier-Server/bin/RepetierServer", "-c", "/usr/local/Repetier-Server/etc/RepetierServer.xml" ]
