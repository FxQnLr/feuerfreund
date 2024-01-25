# feuerfreund
Imbue your discord voice chat with a crackling flame and do other random stuff you don't need

```Dockerfile
services:
  server:
    image: feuerfreund:latest
    container_name: feuerfreund
    environment:
      - FF_TOKEN={string}
      - FF_MC_SERVER_IP={string}
      - FF_EPHEMERAL_REPLIES={bool}
      - FF_DEV_GUILD={int}
    restart: unless-stopped
    volumes:
      - type: bind
        source: ./feuerfreund.log
        target: /feuerfreund.log
```
