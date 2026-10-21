---
icon: lucide/server
---

# `grpc-keyword`

gRPC health check that verifies a keyword appears in the response.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `cache_bust` | `false` | Add cache-busting parameter |
| `description` | `A Monitor` | Monitor description |
| `grpc_body` | `{}` | gRPC request body (JSON) |
| `grpc_enable_tls` | `false` | Enable TLS |
| `grpc_metadata` | `{"authorization":"Bearer token"}` | gRPC metadata headers |
| `grpc_method` | `Check` | gRPC method name |
| `grpc_protobuf` | `health.proto` | Protobuf definition |
| `grpc_service_name` | `grpc.health.v1.Health` | gRPC service name |
| `grpc_url` | `localhost:50051` | gRPC server URL |
| `interval` | `60` | Check interval in seconds |
| `invert_keyword` | `false` | Fail if keyword is found |
| `keyword` | `healthy` | Keyword to search for in the response |
| `max_retries` | `0` | Maximum retries |
| `max_redirects` | `10` | Maximum redirects |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.grpc-health.grpc-keyword.name: "gRPC Health"
  kuma.grpc-health.grpc-keyword.grpc_url: "localhost:50051"
  kuma.grpc-health.grpc-keyword.grpc_service_name: "grpc.health.v1.Health"
  kuma.grpc-health.grpc-keyword.grpc_method: "Check"
  kuma.grpc-health.grpc-keyword.keyword: "SERVING"
```
