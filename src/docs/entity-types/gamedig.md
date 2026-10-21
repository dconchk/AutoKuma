---
icon: lucide/gamepad-2
---

# `gamedig`

Game server monitor using the [GameDig](https://github.com/gamedig/node-gamedig) protocol.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `game` | `minecraft` | Game identifier (see GameDig docs) |
| `gamedig_given_port_only` | `false` | Only use the given port |
| `hostname` | `localhost` | Game server hostname |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries before marking as down |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `0` | Game server port |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.mc.gamedig.name: "Minecraft Server"
  kuma.mc.gamedig.game: "minecraft"
  kuma.mc.gamedig.hostname: "mc.example.com"
  kuma.mc.gamedig.port: "25565"
```
