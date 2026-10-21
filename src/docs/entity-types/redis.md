---
icon: lucide/database
---

# `redis`

Redis connection health monitor.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `database_connection_string` | `redis://localhost:6379` | Redis connection string |
| `description` | `A Monitor` | Monitor description |
| `ignore_tls` | `false` | Ignore TLS errors |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.redis.redis.name: "Redis"
  kuma.redis.redis.database_connection_string: "redis://redis:6379"
```
