---
icon: lucide/folder
---

# `group`

Monitor group. Groups organize monitors in the Uptime Kuma UI. Assign monitors to a group using the `parent_name` AutoKuma property.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the group is active |
| `description` | `A Monitor` | Group description |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Group display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.apps.group.name: "My Applications"

  kuma.myapp.http.name: "My App"
  kuma.myapp.http.parent_name: "apps"
  kuma.myapp.http.url: "https://myapp.example.com"
```
