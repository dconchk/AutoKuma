---
icon: lucide/database
---

# `sqlserver`

Microsoft SQL Server connection health monitor.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `database_connection_string` | `Server=localhost;Database=master;User Id=sa;Password=Password123!;` | SQL Server connection string |
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
  kuma.mssql.sqlserver.name: "SQL Server"
  kuma.mssql.sqlserver.database_connection_string: "Server=mssql;Database=app;User Id=sa;Password=secret;"
  kuma.mssql.sqlserver.query: "SELECT 1"
```
