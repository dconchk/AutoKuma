---
icon: lucide/plug
---

# `port`

TCP port availability monitor. Checks that a TCP connection can be established to a given host and port.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `hostname` | `localhost` | Target hostname |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `0` | TCP port number |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.ssh.port.name: "SSH"
  kuma.ssh.port.hostname: "server.example.com"
  kuma.ssh.port.port: "22"
```
