# yt-dlp
Alpine multiplatform docker image for yt-dlp.


## docker-compose:
```yml
version: '3'
services:
  yt-dlp:
    image: ghcr.io/luckyturtledev/yt-dlp
    volumes:
      - "./data:/data"
    command: yt-dlp -h
```
The `/data` folder must be owned by user id `1000:1000`.
