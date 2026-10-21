---
icon: lucide/globe
---

# `globalping`

Global monitoring via [Globalping](https://globalping.io/). Runs checks from multiple locations worldwide.

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
| `oauth_audience` | `https://api.example.com` | OAuth audience |
| `cache_bust` | `false` | Add cache-busting query parameter |
| `description` | `A Monitor` | Monitor description |
| `dns_resolve_server` | `1.1.1.1` | DNS server to use |
| `dns_resolve_type` | `A` | DNS record type |
| `expected_value` | `success` | Expected value for assertion |
| `expiry_notification` | `false` | Notify on certificate expiry |
| `headers` | `{"X-Api-Key":"secret"}` | Custom request headers |
| `hostname` | `example.com` | Target hostname |
| `ignore_tls` | `false` | Ignore TLS errors |
| `interval` | `60` | Check interval in seconds |
| `ip_family` | `ipv4` | IP family to use (`ipv4`, `ipv6`) |
| `json_path` | `$.status` | JSON path for assertion |
| `json_path_operator` | `==` | JSON path comparison operator |
| `keyword` | `healthy` | Keyword to search for |
| `invert_keyword` | `false` | Fail if keyword is found |
| `location` | `world` | Globalping location |
| `max_retries` | `0` | Maximum retries |
| `method` | `GET` | HTTP method |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `ping_count` | `3` | Number of pings |
| `port` | `80` | Target port |
| `protocol` | `ICMP` | Protocol to use |
| `retry_interval` | `60` | Interval between retries in seconds |
| `subtype` | `ping` | Check subtype (`ping`, `http`, `dns`) |
| `upside_down` | `false` | Invert the status |
| `url` | `https://example.com` | Target URL |
