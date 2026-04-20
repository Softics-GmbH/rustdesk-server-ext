FROM scratch
COPY hbbs /usr/bin/hbbs
COPY hbbr /usr/bin/hbbr
WORKDIR /root
# Admin-API (nur aktiv wenn ADMIN_API_TOKEN env gesetzt):
#   hbbs:21120 -> GET /health, /api/peers/count
#   hbbr:21121 -> GET /health, /api/relay/count
EXPOSE 21120 21121

