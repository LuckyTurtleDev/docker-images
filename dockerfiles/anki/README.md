# Anki Sync Server

The offical selfhosted sync sever for the [anki](https://github.com/ankitects/anki) flashcard program.

* url: [`ghcr.io/luckyturtledev/anki`](https://ghcr.io/luckyturtledev/anki)
* multiplatform: ❌
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
The `SYNC_USER1` environment variabels defines user and password.
Multiple user can be added by using `SYNC_USER2`, `SYNC_USER3`, ... environment variabels.

The docker-compose bind the server to `127.0.0.1:10080`. So the port `10080` can be tunnel through an reverse proxy (strongly recommended for acess above the internett).
If you do not want to use an reverse proxy change `127.0.0.1:10080:8080` to `8080` at the docker-compose.

The `data` folder should be owned by user `1000:1000` or whatever `user` you have define at the docker-compose.

See also https://docs.ankiweb.net/sync-server.html for information about client setup and additonal informations.
