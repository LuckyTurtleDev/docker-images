# repetier-server

Repetier 3D print Server from [repetier-server.com](https://www.repetier-server.com/download-repetier-server/) in a Docker based on Debian.

* url: [`ghcr.io/luckyturtledev/repetier-server`](https://ghcr.io/luckyturtledev/repetier-server:latest)
* multiplatform: ✅
* semver tags: ✅
* version tags: ✅
* auto update: ❌

`docker-compose`:
```yml
version: "3.4"
services:
    repetier-server:
        image: ghcr.io/luckyturtledev/repetier-server:latest
        container_name: repetier-server
        ports:
            - '80:3344'
        volumes:
            - './data:/data'
        devices:
            - /dev/ttyUSB0
        restart: unless-stopped
```
3D Printer should be forwarded at the `devices` section. You may need to mount a different device to the container, as used at the example above. For example `/dev/ttyUSB1` or `/dev/ttyACM0`.
You can list the device with `ls /dev/tty*` before connecting your printer and run the command after connecting it again to find the right device. The device, which is listed additionally, is your printer.
