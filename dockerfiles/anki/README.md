# Anki Sync Server

The official selfhosted sync sever for the [anki](https://github.com/ankitects/anki) flashcard program.

Do not confuse this official server, with the old unofficial (unmaintained?) [community sync server](https://github.com/ankicommunity/anki-sync-server).

* url: [`ghcr.io/luckyturtledev/anki`](https://ghcr.io/luckyturtledev/anki)
* multiplatform: ✅
* semver tags: ❌
* version tags: ✅
* auto update: ✅

docker-compose:
```yml
version: '3.3'
services:
    anki:
        image: ghcr.io/luckyturtledev/anki
        container_name: anki
        environment:
            - SYNC_USER1=user:pass
            - RUST_LOG=info
        ports:
         - 127.0.0.1:10080:8080
        volumes:
            - './data:/data'
        user: 1000:1000
        restart: unless-stopped
```
The `SYNC_USER1` environment variables defines username and password.
Multiple user can be added by using `SYNC_USER2`, `SYNC_USER3`, ... environment variables.

The docker-compose bind the server to `127.0.0.1:10080`. So the port `10080` can be tunnel through an reverse proxy (strongly recommended for access via internet).
If you do not want to use an reverse proxy change `127.0.0.1:10080:8080` to `8080` at the docker-compose and the server will be available at port `8080`.

The `data` folder have to be owned by user `1000:1000` or whatever `user` you have define at the docker-compose.

See also https://docs.ankiweb.net/sync-server.html for information about client setup and additional informations.
