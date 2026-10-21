---
icon: lucide/globe
---

# `http`

HTTP/HTTPS monitor. The most common monitor type. Fetches a URL and checks the response status code.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `authMethod` | `basic` | Authentication method (`null`, `basic`, `oauth2-cc`, `ntlm`, `mtls`) |
| `basic_auth_user` | `monitor` | Basic auth username |
| `basic_auth_pass` | `secret` | Basic auth password |
| `oauth_auth_method` | `client_secret_basic` | OAuth method |
| `oauth_client_id` | `monitor-client` | OAuth client ID |
| `oauth_token_url` | `https://auth.example.com/oauth/token` | OAuth token endpoint |
| `oauth_client_secret` | `secret` | OAuth client secret |
| `oauth_scopes` | `uptime.read` | OAuth scopes |
| `auth_domain` | `EXAMPLE` | NTLM auth domain |
| `auth_workstation` | `WORKSTATION1` | NTLM workstation |
| `tls_cert` | `-----BEGIN CERTIFICATE-----...` | Client TLS certificate (PEM) |
| `tls_key` | `-----BEGIN PRIVATE KEY-----...` | Client TLS private key (PEM) |
| `tls_ca` | `-----BEGIN CERTIFICATE-----...` | Custom CA certificate (PEM) |
| `body` | `{"status":"ok"}` | Request body |
| `cache_bust` | `false` | Add cache-busting query parameter |
| `description` | `A Monitor` | Monitor description |
| `expiry_notification` | `true` | Notify on TLS certificate expiry |
| `headers` | `{"X-Api-Key":"secret"}` | Custom request headers (JSON) |
| `http_body_encoding` | `json` | Body encoding (`json`, `form`, `xml`) |
| `ignore_tls` | `false` | Ignore TLS errors |
| `interval` | `60` | Check interval in seconds |
| `max_redirects` | `10` | Maximum number of redirects to follow |
| `max_retries` | `0` | Maximum retries before marking as down |
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
  kuma.my-site.http.name: "My Website"
  kuma.my-site.http.url: "https://example.com"
  kuma.my-site.http.interval: "60"
  kuma.my-site.http.max_retries: "3"
```
