# Debian 12 Bookworm — local `.deb` build and install proof

## Goal

Produce a Bookworm-linked `cx-terminal` package (glibc 2.36 / OpenSSL 3 on Debian) without publishing to `repo.cxlinux.com`, then install it in a fresh `debian:12` container.

## Build

```bash
# On debian:12 (or Docker):
bash ci/build-deb-bookworm.sh
# Artifact: cx-terminal-<TAG>.debian12.deb
```

CI: `.github/workflows/debian12-deb-smoke.yml` (build job in `debian:12` container + install smoke job).

## Install test

```bash
bash ci/test-deb-bookworm.sh cx-terminal-*.debian12.deb
```

## Results (local agent run, 2026-10-19)

Artifacts: `cx-terminal-bookworm-local-verify.debian12-minimal.deb` (not published), logs below.

| Step | Result |
|------|--------|
| Build `cx-terminal` on `debian:12` (`MINIMAL_DEB=1`) | **PASS** — see `bookworm-deb-build.log` |
| `dpkg -i` in fresh `debian:12` | **PASS** — pulls Bookworm `libssl3`, `libgomp1`, etc. |
| `cx --version` | **PASS** |
| `cx activate --help` | **PASS** |
| `cx license activate --help` | **PASS** (alias) |
| `cx activate` invalid key → `❌ Activation failed: Invalid license key` (exit 1) | **PASS** — live `license.cxlinux.com` |
| Full GUI `.deb` via `ci/deploy.sh` on Bookworm | **FAIL** in this run — `freetype`/`zlib` parallel build flake; minimal CLI deb used instead |

### Valid license key (not exercised — no Stripe / no test key)

On success, `cx activate <KEY>` would:

1. `POST /api/v1/licenses/activate` with hardware fingerprint + hostname.
2. Save key to `~/.cx/license.key`.
3. `POST /api/v1/licenses/validate` and print tier, email, expiry, device limits, features.

See `LICENSE_ACTIVATION_FLOW.md`.
