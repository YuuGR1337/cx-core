# APT clean-install verification (2026-10-19)

Commands follow [README.md](../../README.md) **APT Repository (Recommended)** exactly.

## Ubuntu 24.04 — **PASS**

```bash
docker run --rm ubuntu:24.04 bash -c '<install script>'
```

| Step | Result |
|------|--------|
| `curl -fsSL https://repo.cxlinux.com/key.gpg \| gpg --dearmor -o /etc/apt/keyrings/cxlinux.gpg` | PASS |
| Add `deb [signed-by=…] https://repo.cxlinux.com/apt stable main` | PASS |
| `apt-get install cx-terminal` | PASS |
| Package version | `20260223-145346-e97bbc7b` (deb `0.3.2` lineage) |
| `cx-terminal --version` | `cx-terminal 20260223-145346-e97bbc7b` |
| `cx-terminal --help` | PASS (ask, install, setup, what, fix, ai, …) |
| `cx --version` / `cx --help` | **FAIL** — no `/usr/bin/cx` on current published package (fixed in `ci/deploy.sh` postinst for next release) |
| `cx-terminal activate` | **FAIL** on published build — subcommand missing at top level (fixed in `wezterm/src/main.rs` for next release) |
| `cx-terminal cli activate` | Reaches license server after mux bypass fix in this PR; published build fails spawning `wezterm-mux-server` (package ships `cx-mux-server`) |

Full transcript: [apt-install-ubuntu-24.04.log](./apt-install-ubuntu-24.04.log)

## Debian 12 (bookworm) — **FAIL**

```bash
docker run --rm debian:12 bash -c '<install script>'
```

`apt-get install cx-terminal` errors:

```
cx-terminal : Depends: libc6 (>= 2.39) but 2.36-9+deb12u14 is to be installed
              Depends: libssl3t64 (>= 3.0.0) but it is not installable
              Depends: libx11-xcb1 (>= 2:1.8.7) but it is not going to be installed
E: Unable to correct problems, you have held broken packages.
```

Full transcript: [apt-install-debian-12.log](./apt-install-debian-12.log)

**Blocker:** publish a Bookworm-built `.deb` (or lower glibc/openssl deps) before advertising Debian 12 APT support.

## CI

Workflow [`.github/workflows/apt-install-smoke.yml`](../../.github/workflows/apt-install-smoke.yml) runs the same steps on `ubuntu-24.04` (required) and `debian:12` in Docker (`continue-on-error: true` until packaging is fixed).
