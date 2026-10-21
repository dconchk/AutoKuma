---
icon: lucide/mail
---

# `smtp`

SMTP server availability monitor. Connects to an SMTP server and checks that it responds correctly.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `hostname` | `localhost` | SMTP server hostname |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `25` | SMTP server port |
| `security` | `nostarttls` | Connection security (`secure`, `nostarttls`, `starttls`) |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.mail.smtp.name: "Mail Server"
  kuma.mail.smtp.hostname: "mail.example.com"
  kuma.mail.smtp.port: "587"
  kuma.mail.smtp.security: "starttls"
```
