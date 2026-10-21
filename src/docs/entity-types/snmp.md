---
icon: lucide/activity
---

# `snmp`

SNMP OID monitor. Queries an SNMP agent for an OID value and compares it against an expected value.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `expected_value` | `1` | Expected value from the OID |
| `hostname` | `localhost` | SNMP agent hostname |
| `interval` | `60` | Check interval in seconds |
| `json_path` | `$.value` | JSON path to extract the value |
| `json_path_operator` | `==` | Comparison operator (`>`, `>=`, `<`, `<=`, `!=`, `==`, `contains`) |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `oid` | `1.3.6.1.2.1.1.3.0` | OID to query |
| `parent` | `0` | Parent group ID |
| `radius_password` | `public` | SNMP community string |
| `port` | `161` | SNMP agent port |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |
| `version` | `2c` | SNMP version (`1`, `2c`) |

## Example

```yaml
labels:
  kuma.switch.snmp.name: "Network Switch"
  kuma.switch.snmp.hostname: "switch.example.com"
  kuma.switch.snmp.oid: "1.3.6.1.2.1.1.3.0"
  kuma.switch.snmp.version: "2c"
  kuma.switch.snmp.radius_password: "public"
```
