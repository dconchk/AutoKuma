---
icon: lucide/shield
---

# `radius`

RADIUS authentication health monitor.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `hostname` | `localhost` | RADIUS server hostname |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `0` | RADIUS server port |
| `radius_called_station_id` | `AP-01` | Called station ID |
| `radius_calling_station_id` | `client-01` | Calling station ID |
| `radius_password` | `password` | RADIUS user password |
| `radius_secret` | `secret` | RADIUS shared secret |
| `radius_username` | `monitor` | RADIUS username |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |
