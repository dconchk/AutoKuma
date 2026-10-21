---
icon: lucide/list
---

# Entity Types

AutoKuma supports all monitor types available in Uptime Kuma, plus a few additional entity types for managing notifications, tags, and Docker hosts.

## AutoKuma-Specific Properties

The following properties are handled internally by AutoKuma and are not sent to Uptime Kuma:

| Property | Example | Description |
|----------|---------|-------------|
| `parent_name` | `apps` | AutoKuma ID of the parent group |
| `notification_name_list` | `["matrix", "discord"]` | AutoKuma IDs of notification providers to enable |
| `tag_names` | `[{"name": "mytag", "value": "A value"}]` | Tags to attach to the monitor |
| `docker_host_name` | `local_socket` | AutoKuma ID of the Docker host for a docker monitor |
| `create_paused` | `false` | If `true`, new monitors are added in a paused state (existing monitors are not affected) |

## `docker_host`

| Property | Example |
|----------|---------|
| `connection_type` | `socket` or `tcp` |
| `host` or `path` | `/var/run/docker.sock` |

## `notification`

| Property | Example | Notes |
|----------|---------|-------|
| `active` | `true` | |
| `is_default` | `true` | Only used by the WebUI; AutoKuma does not respect this setting |
| `config` | `{"type": "matrix", ...}` | Provider-specific settings. Use `kuma notification get` to inspect options for your provider. |

## Monitor Types

| Type | Description |
|------|-------------|
| [dns](dns.md) | DNS resolution check |
| [docker](docker.md) | Docker container health check |
| [gamedig](gamedig.md) | Game server monitor |
| [globalping](globalping.md) | Global ping/HTTP check via Globalping |
| [group](group.md) | Monitor group |
| [grpc-keyword](grpc-keyword.md) | gRPC keyword check |
| [http](http.md) | HTTP/HTTPS monitor |
| [json-query](json-query.md) | HTTP monitor with JSON path assertion |
| [kafka-producer](kafka-producer.md) | Kafka producer health check |
| [keyword](keyword.md) | HTTP monitor with keyword check |
| [mongodb](mongodb.md) | MongoDB connection check |
| [mqtt](mqtt.md) | MQTT broker check |
| [mysql](mysql.md) | MySQL/MariaDB connection check |
| [ping](ping.md) | ICMP ping monitor |
| [port](port.md) | TCP port check |
| [postgres](postgres.md) | PostgreSQL connection check |
| [push](push.md) | Push (heartbeat) monitor |
| [radius](radius.md) | RADIUS authentication check |
| [real-browser](real-browser.md) | Headless browser check |
| [redis](redis.md) | Redis connection check |
| [steam](steam.md) | Steam game server check |
| [sqlserver](sqlserver.md) | SQL Server connection check |
| [tailscale-ping](tailscale-ping.md) | Tailscale node ping |
| [smtp](smtp.md) | SMTP server check |
| [snmp](snmp.md) | SNMP OID check |
| [rabbitmq](rabbitmq.md) | RabbitMQ health check |
