---
icon: lucide/container
---

# `docker`

Docker container health monitor. Checks the status of a Docker container.

## Properties

| Property | Example | Description |
|----------|---------|-------------|
| `accepted_statuscodes` | `200-299` | Accepted HTTP status codes |
| `active` | `true` | Whether the monitor is active |
| `description` | `A Monitor` | Monitor description |
| `docker_container` | `nginx` | Container name to monitor |
| `docker_host` | `1` | ID of the Docker host (use `docker_host_name` for AutoKuma-managed hosts) |
| `interval` | `60` | Check interval in seconds |
| `max_retries` | `0` | Maximum retries before marking as down |
| `name` | `Example` | Monitor display name |
| `parent` | `0` | Parent group ID |
| `retry_interval` | `60` | Interval between retries in seconds |
| `upside_down` | `false` | Invert the status |

## Example

```yaml
labels:
  kuma.my-nginx.docker.name: "nginx"
  kuma.my-nginx.docker.docker_container: "nginx"
  kuma.my-nginx.docker.docker_host_name: "local"
```
