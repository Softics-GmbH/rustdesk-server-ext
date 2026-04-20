# Changelog

Changes of this fork against upstream
[`lejianwen/rustdesk-server`](https://github.com/lejianwen/rustdesk-server).

The upstream project keeps its own version history in its README and release
tags — this file only documents Softics-GmbH-specific additions.

## Unreleased

### Added

- **Admin-API (count-only)** — new HTTP endpoints on both hbbs and hbbr for
  operational monitoring without exposing personally-identifiable data:
  - `hbbs:21120`: `GET /health`, `GET /api/peers/count` (active peer count
    within a configurable heartbeat threshold).
  - `hbbr:21121`: `GET /health`, `GET /api/relay/count` (active paired relay
    sessions — lower bound, direct P2P is server-invisible).
  - Authentication via `X-Admin-Token` header with constant-time compare.
  - Opt-in: endpoints stay closed unless `ADMIN_API_TOKEN` is set.
  - Configurable threshold via `PEER_ACTIVE_THRESHOLD_SECONDS` (default 120).
  - `docker-classic/Dockerfile` exposes `21120` and `21121`.
- `NOTICE` file with Softics-GmbH copyright and upstream attribution chain
  (rustdesk/rustdesk-server → lejianwen/rustdesk-server → this fork).
- `README.md` Softics fork section at the top (English, fork purpose, image
  coordinates, AGPL-3.0 source-availability statement).
- `.github/workflows/softics-build.yml` — amd64-only CI that builds on every
  push to `main`, every PR, and every `v*` tag; publishes to
  `ghcr.io/softics-gmbh/rustdesk-server-ext` with `:latest`, `:sha-<short>`,
  and (on tag push) `:<version>` + `:<git-ref>` tags.

### Removed

- Accidental `test_push.txt` left over from initial fork setup (PR #1).

### Preserved from upstream

Upstream workflows (`build.yaml`, `ghcr.yml`, `build-test.yaml`, `test.yml`)
and docker definitions are kept unmodified to ease future upstream merges.
