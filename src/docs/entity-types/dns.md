---
icon: lucide/network
---

# `dns`

DNS resolution monitor. Checks that a hostname resolves to an expected value using a specified DNS server.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `dns_resolve_server` | `1.1.1.1` | DNS server to use for resolution |
| `dns_resolve_type` | `A` | DNS record type to resolve |
| `hostname` | `localhost` | Hostname to resolve |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries before marking as down |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `port` | `0` | Port (if applicable) |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status (up becomes down) |

## Example

```yaml
labels:
  kuma.my-dns.dns.name: "DNS Check"
  kuma.my-dns.dns.hostname: "example.com"
  kuma.my-dns.dns.dns_resolve_server: "1.1.1.1"
  kuma.my-dns.dns.dns_resolve_type: "A"
  kuma.my-dns.dns.interval: "60"
```
