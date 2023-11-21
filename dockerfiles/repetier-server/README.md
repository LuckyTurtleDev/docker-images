# docker-repetier-server

Repetier Server from [repetier-server.com](https://www.repetier-server.com/download-repetier-server/) in a Docker based on Debian.

## Summary

- Peristant data are located in `/data`. You can either use a bind-mount or create a volume.
- Port `3344` is the exposed port for the Web UI
- USB printers should be forwarded with `--device` argument

## Usage

`docker-compose:`
```yml
version: "3.4"
services:
    repetier-server:
        image: ghcr.io/luckyturtledev/yt-dlp:latest
        container_name: repetier-server
        ports:
            - '80:3344'
        volumes:
            - './data:/data'
        devices:
            - /dev/ttyUSB0
        restart: unless-stopped
```
You may need to mount a different device to the container, as example `/dev/ttyUSB1` or `/dev/ttyACM0`.
You can list the device with `ls /dev/tty*` before connecting your printer and run the command after connecting it again to find the right device. The device, which is listed additionally, is your printer.
