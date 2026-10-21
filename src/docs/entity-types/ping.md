---
icon: lucide/activity
---

# `ping`

ICMP ping monitor. Sends ping packets to a host and checks for a response.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `hostname` | `localhost` | Hostname or IP address to ping |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `packet_size` | `56` | ICMP packet size in bytes |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.router.ping.name: "Router"
  kuma.router.ping.hostname: "192.168.1.1"
```
