---
icon: lucide/home
---

# AutoKuma

AutoKuma is a utility that automates the creation of [Uptime Kuma](https://uptime.kuma.pet/) monitors based on Docker container labels, files, and more. With AutoKuma, you can eliminate the need for manual monitor creation in the Uptime Kuma UI.

## What is AutoKuma?

AutoKuma watches your infrastructure and automatically keeps Uptime Kuma in sync. Add a label to a Docker container, drop a file in a directory, or define a Kubernetes resource, and the corresponding monitor appears in Uptime Kuma. Remove the source and the monitor disappears.

## Components

This repository contains three separate tools:

| Component | Description |
|-----------|-------------|
| [AutoKuma](autokuma/index.md) | The main daemon that syncs monitors from Docker labels, files, and more |
| [Kuma CLI](kuma-cli.md) | A command-line interface for managing Uptime Kuma directly |
| [Kuma Client](kuma-client.md) | A Rust library for interacting with the Uptime Kuma SocketIO API |

## Supported Sources

AutoKuma can source monitor configurations from several places:

| Source | Description | Support |
|--------|-------------|---------|
| Docker | Monitors are sourced from container labels | Stable |
| Files | Monitors are sourced from `.json` or `.toml` files | Stable |
| Docker Swarm | Monitors from service and container labels | Community |
| Kubernetes | Monitors from custom resources, files, or Docker labels | Community |

## Quick Example

Add labels to any Docker container and AutoKuma creates the monitor automatically:

```yaml
services:
  my-app:
    image: my-app:latest
    labels:
      kuma.my-app.http.name: "My App"
      kuma.my-app.http.url: "https://my-app.example.com"
```

That is all that is needed. AutoKuma picks up the labels and creates the monitor in Uptime Kuma.

[Get started with AutoKuma](autokuma/installation.md){ .md-button .md-button--primary }
[Browse entity types](entity-types/index.md){ .md-button }
