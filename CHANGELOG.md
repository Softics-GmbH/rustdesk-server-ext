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

### Changed

- **Upstream merge: `rustdesk/rustdesk-server` 1.1.16** (2026-10-27). The fork
  previously sat on an upstream snapshot from March 2025 (`c650217`). Relevant
  upstream changes now included:
  - **CVE-2026-30784** — UDP `PunchHoleRequest` is no longer handled by hbbs
    (unauthenticated reflection/amplification vector). Clients use the
    TCP/WebSocket path, which stays unchanged.
  - i32 overflow fix: peers offline for 24.9–49.7 days were wrongly reported
    online (#676). The fork's `peers_online_state()` helper was switched to
    `i64` accordingly.
  - `mio` 0.8.5 → 0.8.11 security update (#633).
  - Failed key authentication is logged with `warn` on hbbs and hbbr
    (fail2ban / CrowdSec friendly, #435).
  - New console command `punch-requests(pr)` listing recent punch-hole
    requests.
  - `ENV HOME=/root` in `docker-classic/Dockerfile` (rootless support, #612).
  - Crate version is now `1.1.16`.
- `libs/hbb_common` stays on `d6b1497` (newer than upstream's `83419b6`,
  required for the WebSocket support inherited from lejianwen).

### Removed

- Upstream deleted `.github/workflows/ghcr.yml`, `.github/workflows/test.yml`
  and the translated `README-DE.md` / `README-NL.md` / `README-TW.md`; the
  fork follows that deletion.
- Accidental `test_push.txt` left over from initial fork setup (PR #1).

### Preserved from upstream

Upstream workflows (`build.yaml`, `build-test.yaml`) and docker definitions
are kept unmodified to ease future upstream merges.
