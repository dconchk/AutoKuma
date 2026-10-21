<div align="center" width="100%">
    <img src="./logo.svg" height="196" alt="" />
</div>

#### [](HEADER)

<div align="center" width="100%">
    <p>
        <a href="https://github.com/BigBoot/AutoKuma/actions"><img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/BigBoot/AutoKuma/docker-build-push.yml?style=flat&logo=rust&link=https%3A%2F%2Fgithub.com%2FBigBoot%2FAutoKuma%2Factions"></a>
        <a href="https://github.com/BigBoot/AutoKuma/releases/latest"><img alt="GitHub Tag" src="https://img.shields.io/github/v/tag/BigBoot/AutoKuma?logo=github&label=latest"></a>
        <a href="https://ghcr.io/bigboot/autokuma"><img alt="GHCR Tag" src="https://img.shields.io/github/v/tag/BigBoot/AutoKuma?logo=docker&logoColor=white&label=GHCR"></a>
    </p>
    <p>
        <b>
            <a href="#autokuma--">AutoKuma</a>
            &nbsp&nbsp
            <a href="#kuma-cli---">Kuma CLI</a>
            &nbsp&nbsp
            <a href="#kuma-client--">Kuma Client</a>
            &nbsp&nbsp
            <a href="https://autokuma.bigboot.dev/dev/">Documentation</a>
            &nbsp&nbsp
            <a href="https://autokuma-playground.bigboot.dev">Playground</a>
        </b>
    </p>
</div>


# AutoKuma 🐻 <a href="https://crates.io/crates/autokuma"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/autokuma?logo=rust&color=blue"></a>

AutoKuma is a utility that automates the creation of Uptime Kuma monitors based on Docker container labels. With AutoKuma, you can eliminate the need for manual monitor creation in the Uptime Kuma UI.

## Supported Sources

| Source | Description | Support |
|--------|-------------|---------|
| Docker | Monitors are sourced from container labels | ✅ |
| Files | Monitors are sourced from `.json`/`.toml` files | ✅ |
| Docker Swarm | Monitors can be sourced from service and/or container labels | ⚠️* |
| Kubernetes | Monitors are sourced from CR, see the CRDs in `autokuma/kubernetes/crds-autokuma.yml`. Additionally the `Files` and `Docker` provider might be used depending on your setup | ⚠️* |

*These sources are supported on an as-is basis as I'm currently not running any of them (they are basically looking for a maintainer, please get in contact if you'd like to adopt one or add support for another source).

## Quick Start 🚀

The fastest way to get going is with Docker Compose:

```yaml
version: '3'

services:
  autokuma:
    image: ghcr.io/bigboot/autokuma:latest
    restart: unless-stopped
    environment:
      AUTOKUMA__KUMA__URL: http://uptime-kuma:3001
      AUTOKUMA__KUMA__USERNAME: <username>
      AUTOKUMA__KUMA__PASSWORD: <password>
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - autokuma-data:/data

volumes:
  autokuma-data:
```

Then add labels to any container you want to monitor:

```yaml
labels:
  kuma.my-app.http.name: "My App"
  kuma.my-app.http.url: "https://my-app.example.com"
```

AutoKuma picks up the labels and creates the monitor in Uptime Kuma automatically.

For full installation options (prebuilt binaries, platforms, dev builds) see the [Installation docs](https://autokuma.bigboot.dev/dev/autokuma/installation/).

## Usage 💡

Labels follow this format:

```
<prefix>.<id>.<type>.<setting>: <value>
```

- `<prefix>` - Default is `kuma`, configurable via `AUTOKUMA__DOCKER__LABEL_PREFIX`.
- `<id>` - A unique identifier for the monitor.
- `<type>` - The monitor type (e.g. `http`, `ping`, `dns`).
- `<setting>` - The property to set.

The docs cover everything else: [groups](https://autokuma.bigboot.dev/dev/autokuma/usage/#groups), [notifications](https://autokuma.bigboot.dev/dev/autokuma/usage/#notifications), [tags](https://autokuma.bigboot.dev/dev/autokuma/usage/#tags), [templating](https://autokuma.bigboot.dev/dev/autokuma/templating/), [snippets](https://autokuma.bigboot.dev/dev/autokuma/snippets/), [static monitors](https://autokuma.bigboot.dev/dev/autokuma/static-monitors/), and all [entity types](https://autokuma.bigboot.dev/dev/entity-types/).

## Configuration 🔧

At minimum you need to point AutoKuma at your Uptime Kuma instance:

```bash
AUTOKUMA__KUMA__URL=http://localhost:3001
AUTOKUMA__KUMA__USERNAME=<username>
AUTOKUMA__KUMA__PASSWORD=<password>
```

See the [Configuration docs](https://autokuma.bigboot.dev/dev/autokuma/configuration/) for the full reference, including TLS, Docker host options, secret files, and config file locations.


# Kuma CLI 🤖 <a href="https://crates.io/crates/kuma-cli"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/kuma-cli?logo=rust&color=blue"></a> [![kuma](https://snapcraft.io/kuma/badge.svg)](https://snapcraft.io/kuma)

Kuma CLI is a command-line tool for managing Uptime Kuma directly from the terminal.

## Commands

| Resource | Subcommands |
|----------|-------------|
| `monitor` | `add`, `delete`, `edit`, `list`, `get`, `pause`, `resume` |
| `tag` | `add`, `delete`, `edit`, `ls`, `get` |
| `notification` | `add`, `delete`, `edit`, `ls`, `get` |
| `maintenance` | `add`, `delete`, `edit`, `ls`, `get`, `pause`, `resume` |
| `status-page` | `add`, `delete`, `edit`, `ls`, `get` |
| `docker-host` | `add`, `delete`, `edit`, `ls`, `get`, `test` |

## How to Install 📦

Binaries for Windows, Linux and macOS are available on [GitHub Releases](https://github.com/BigBoot/AutoKuma/releases/latest). You can also install via Snap or Cargo:

```bash
sudo snap install kuma
# or
cargo install --git https://github.com/BigBoot/AutoKuma.git kuma-cli
```

See the [Kuma CLI docs](https://autokuma.bigboot.dev/dev/kuma-cli/) for usage, all options, and configuration.


# Kuma Client 🧑‍💻 <a href="https://crates.io/crates/kuma-client"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/kuma-client?logo=rust&color=blue"></a>

`kuma-client` is a Rust crate that provides a client library for interacting with the Uptime Kuma SocketIO API.

Please take a look at [the examples](kuma-client/examples/) and the [documentation](https://docs.rs/kuma-client/latest/kuma_client/) for further details.

# Contributing 👥

Contributions to AutoKuma are welcome! Feel free to open issues, submit pull requests, or provide feedback.

# License 📜

AutoKuma is released under the [MIT License](LICENSE).
