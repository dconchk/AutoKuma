---
icon: lucide/wifi
---

# `tailscale-ping`

Tailscale node ping monitor. Pings a Tailscale network node.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.ts-node.tailscale-ping.name: "Tailscale Node"
  kuma.ts-node.tailscale-ping.hostname: "my-node.example.ts.net"
```
