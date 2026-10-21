---
icon: lucide/gamepad
---

# `steam`

Steam game server monitor.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `hostname` | `localhost` | Game server hostname |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `0` | Game server port |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.steam-server.steam.name: "Game Server"
  kuma.steam-server.steam.hostname: "game.example.com"
  kuma.steam-server.steam.port: "27015"
```
