---
icon: lucide/database
---

# `mongodb`

MongoDB connection health monitor. Connects to MongoDB and runs a command to verify availability.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `command` | `{"ping":1}` | MongoDB command to run |
| `database_connection_string` | `mongodb://localhost:27017/admin` | MongoDB connection string |
| `description` | `A Monitor` | Monitor description |
| `expected_value` | `1` | Expected result value |
| `interval` | `60` | Check interval in seconds |
| `json_path` | `$.ok` | JSON path to extract from the result |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.mongo.mongodb.name: "MongoDB"
  kuma.mongo.mongodb.database_connection_string: "mongodb://user:pass@mongo:27017/admin"
```
