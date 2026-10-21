---
icon: lucide/package-open
render_macros: true
---

# Installation

## Docker (Recommended)

AutoKuma is available as a Docker image on the [GitHub Container Registry](https://github.com/BigBoot/AutoKuma/pkgs/container/autokuma).

```bash
docker pull ghcr.io/bigboot/autokuma:{{ autokuma_latest_release }}
```

!!! tip "Using the `latest` tag"
    You can also use the `latest` tag, which always points to the most recent stable release. However, a versioned tag like `{{ autokuma_latest_release }}` is preferred - it ensures reproducible deployments and keeps you in explicit control of when to update.

    ```bash
    docker pull ghcr.io/bigboot/autokuma:latest
    ```

!!! warning "Dev builds"
    Development builds (`master` tag) may contain breaking changes without warning. Do not use them in production.

    ```bash
    docker pull ghcr.io/bigboot/autokuma:master
    ```

??? note "Uptime Kuma V1"
    For Uptime Kuma V1, use the following image instead:

    ```bash
    docker pull ghcr.io/bigboot/autokuma:uptime-kuma-v1-latest
    ```

## Docker Compose

Here is a minimal `docker-compose.yml` to get AutoKuma running alongside Uptime Kuma:

```yaml
version: '3'

services:
  autokuma:
    image: ghcr.io/bigboot/autokuma:{{ autokuma_latest_release }}
    restart: unless-stopped
    environment:
      AUTOKUMA__KUMA__URL: http://uptime-kuma:3001
      # AUTOKUMA__KUMA__USERNAME: <username>
      # AUTOKUMA__KUMA__PASSWORD: <password>
      # AUTOKUMA__KUMA__MFA_TOKEN: <token>
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - autokuma-data:/data

volumes:
  autokuma-data:
```

## Prebuilt Binaries

Prebuilt binaries for Windows, Linux, and macOS are available on the [GitHub Releases](https://github.com/BigBoot/AutoKuma/releases/latest) page.

## Build from Source

```bash
cargo install --git https://github.com/BigBoot/AutoKuma.git autokuma
```
