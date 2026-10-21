---
icon: lucide/message-square
---

# `kafka-producer`

Kafka producer health monitor. Sends a test message to a Kafka topic and verifies it is accepted.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `interval` | `60` | Check interval in seconds |
| `kafka_producer_allow_auto_topic_creation` | `true` | Allow automatic topic creation |
| `kafka_producer_brokers` | `["localhost:9092"]` | Kafka broker addresses |
| `kafka_producer_message` | `autokuma test message` | Message to send |
| `kafka_producer_sasl_options` | `{"mechanism":"plain","username":"user","password":"pass"}` | SASL authentication options |
| `kafka_producer_ssl` | `false` | Enable SSL |
| `kafka_producer_topic` | `monitor-events` | Kafka topic name |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.kafka.kafka-producer.name: "Kafka"
  kuma.kafka.kafka-producer.kafka_producer_brokers: '["kafka:9092"]'
  kuma.kafka.kafka-producer.kafka_producer_topic: "monitor-events"
```
