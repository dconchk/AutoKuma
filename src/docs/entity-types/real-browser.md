---
icon: lucide/monitor
---

# `real-browser`

Headless browser monitor. Uses a remote Chrome instance to load a page and check for successful rendering.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `remote_browser` | `ws://localhost:3000` | WebSocket URL of the remote browser |
| `remote_browsers_toggle` | `true` | Enable remote browser |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |
| `url` | `https://example.com` | URL to load in the browser |

## Example

```yaml
labels:
  kuma.browser.real-browser.name: "Browser Check"
  kuma.browser.real-browser.url: "https://example.com"
  kuma.browser.real-browser.remote_browser: "ws://chrome:3000"
```
