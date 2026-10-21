---
icon: lucide/radio
---

# `mqtt`

MQTT broker monitor. Subscribes to a topic and checks for an expected message or keyword.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `expected_value` | `online` | Expected value to match |
| `interval` | `60` | Check interval in seconds |
| `hostname` | `localhost` | MQTT broker hostname |
| `json_path` | `$.status` | JSON path to extract from the message |
| `json_path_operator` | `==` | Comparison operator (`>`, `>=`, `<`, `<=`, `!=`, `==`, `contains`) |
| `max_retries` | `0` | Maximum retries |
| `mqtt_check_type` | `keyword` | Check type (`keyword`, `json-query`) |
| `mqtt_password` | `mqtt-pass` | MQTT broker password |
| `mqtt_success_message` | `online` | Expected success message |
| `mqtt_topic` | `sensors/status` | MQTT topic to subscribe to |
| `mqtt_username` | `mqtt-user` | MQTT broker username |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `0` | MQTT broker port |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.mqtt-sensor.mqtt.name: "Sensor Status"
  kuma.mqtt-sensor.mqtt.hostname: "mqtt.example.com"
  kuma.mqtt-sensor.mqtt.mqtt_topic: "sensors/status"
  kuma.mqtt-sensor.mqtt.mqtt_success_message: "online"
```
