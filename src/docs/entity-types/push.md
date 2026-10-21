---
icon: lucide/send
---

# `push`

Push (heartbeat) monitor. Your service calls a unique push URL at regular intervals. If the URL is not called within the expected interval, the monitor goes down.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `interval` | `60` | Expected heartbeat interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `push_token` | `4Gdp9cHeNu7MHZ6P6RPiiVbHgSdEHJz7` | Auto-generated push token |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.my-cron.push.name: "Nightly Backup"
  kuma.my-cron.push.interval: "86400"
```
