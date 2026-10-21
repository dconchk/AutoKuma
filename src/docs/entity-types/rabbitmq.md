---
icon: lucide/rabbit
---

# `rabbitmq`

RabbitMQ health monitor. Checks the RabbitMQ management API for node health.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `nodes` | `["http://localhost:15672"]` | RabbitMQ management API URLs |
| `parent` | `0` | Parent group ID |
| `password` | `guest` | RabbitMQ password |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |
| `username` | `guest` | RabbitMQ username |

## Example

```yaml
labels:
  kuma.mq.rabbitmq.name: "RabbitMQ"
  kuma.mq.rabbitmq.nodes: '["http://rabbitmq:15672"]'
  kuma.mq.rabbitmq.username: "monitor"
  kuma.mq.rabbitmq.password: "secret"
```
