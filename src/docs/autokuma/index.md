---
icon: lucide/bot
---

# AutoKuma

AutoKuma is a daemon that watches your infrastructure and keeps Uptime Kuma monitors in sync. It reads monitor definitions from Docker container labels, static files, Docker Swarm services, and Kubernetes resources.

## How It Works

AutoKuma periodically scans configured sources for monitor definitions. Each definition has a unique **AutoKuma ID** that AutoKuma uses to track which monitors it manages (via an Uptime Kuma tag). When a definition disappears, AutoKuma can delete or keep the monitor, depending on your configuration.

## Supported Platforms

| Platform | Architecture | Docker Images | Prebuilt Binaries |
|----------|--------------|:-------------:|:-----------------:|
| Windows | x64 | No | Yes |
| Linux | x64 | Yes | Yes |
| Linux | arm64 | Experimental | No |
| Mac | arm64 | No | Experimental |

## Next Steps

- [Installation](installation.md) - Get AutoKuma running
- [Configuration](configuration.md) - All configuration options
- [Usage](usage.md) - Label format, groups, notifications, and more
- [Templating](templating.md) - Dynamic values using Tera templates
- [Snippets](snippets.md) - Reusable monitor definitions
- [Static Monitors](static-monitors.md) - File-based monitor definitions
