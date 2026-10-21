---
icon: lucide/code-2
---

# `json-query`

HTTP monitor that fetches a URL and asserts a JSON path expression against the response.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `authMethod` | `basic` | Authentication method |
| `basic_auth_user` | `monitor` | Basic auth username |
| `basic_auth_pass` | `secret` | Basic auth password |
| `oauth_auth_method` | `client_secret_basic` | OAuth method |
| `oauth_client_id` | `monitor-client` | OAuth client ID |
| `oauth_token_url` | `https://auth.example.com/oauth/token` | OAuth token endpoint |
| `oauth_client_secret` | `secret` | OAuth client secret |
| `oauth_scopes` | `uptime.read` | OAuth scopes |
| `auth_domain` | `EXAMPLE` | NTLM auth domain |
| `auth_workstation` | `WORKSTATION1` | NTLM workstation |
| `tls_cert` | `-----BEGIN CERTIFICATE-----...` | Client TLS certificate |
| `tls_key` | `-----BEGIN PRIVATE KEY-----...` | Client TLS private key |
| `tls_ca` | `-----BEGIN CERTIFICATE-----...` | Custom CA certificate |
| `body` | `{"status":"ok"}` | Request body |
| `description` | `A Monitor` | Monitor description |
| `expected_value` | `up` | Expected value of the JSON path result |
| `expiry_notification` | `true` | Notify on TLS certificate expiry |
| `headers` | `{"X-Api-Key":"secret"}` | Custom request headers |
| `http_body_encoding` | `json` | Body encoding (`json`, `form`, `xml`) |
| `ignore_tls` | `false` | Ignore TLS errors |
| `interval` | `60` | Check interval in seconds |
| `json_path` | `$.status` | JSON path expression |
| `json_path_operator` | `==` | Comparison operator (`>`, `>=`, `<`, `<=`, `!=`, `==`, `contains`) |
| `max_redirects` | `10` | Maximum redirects |
| `max_retries` | `0` | Maximum retries |
| `method` | `GET` | HTTP method |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `proxy_id` | `1` | Proxy ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `timeout` | `48` | Request timeout in seconds |
| `upside_down` | `false` | Invert the status |
| `url` | `https://example.com` | URL to monitor |

## Example

```yaml
labels:
  kuma.api-health.json-query.name: "API Health"
  kuma.api-health.json-query.url: "https://api.example.com/health"
  kuma.api-health.json-query.json_path: "$.status"
  kuma.api-health.json-query.json_path_operator: "=="
  kuma.api-health.json-query.expected_value: "ok"
```
