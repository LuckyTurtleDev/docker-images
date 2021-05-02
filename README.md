# youtube-dl-cron docker image
Simple multiplatform docker image for youtube-dl with crond. Based on alpine.

example docker-compose:
```yml
version: '3'
services:
  youtube-dl-cron:
    image: registry.gitlab.com/lukas1818/docker-youtube-dl-cron:latest
    volumes:
      - "./data:/data"
      - "./tasks.cron:/tasks.cron:ro"
    restart: unless-stopped
```
