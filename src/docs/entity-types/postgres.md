---
icon: lucide/database
---

# `postgres`

PostgreSQL connection health monitor.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `database_connection_string` | `postgres://postgres:password@localhost:5432/postgres` | PostgreSQL connection string |
| `description` | `A Monitor` | Monitor description |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `query` | `SELECT 1` | SQL query to run as a health check |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.pg.postgres.name: "PostgreSQL"
  kuma.pg.postgres.database_connection_string: "postgres://user:pass@postgres:5432/app"
  kuma.pg.postgres.query: "SELECT 1"
```
