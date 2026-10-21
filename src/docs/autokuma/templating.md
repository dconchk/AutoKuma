---
icon: lucide/braces
---

# Templating

AutoKuma uses [Tera](https://keats.github.io/tera/) templates in label values and [Snippets](snippets.md). Templates let you build monitor definitions dynamically from container metadata.

!!! tip "Try it in the browser"
    The [AutoKuma Playground](https://autokuma-playground.bigboot.dev/) lets you test templates and snippets interactively.

## Available Variables

### Docker Containers

| Variable | Description | Example |
|----------|-------------|---------|
| `container_id` | Full container ID | `92366941fb1f...` |
| `image_id` | SHA256 of the container image | `sha256:c2e38600...` |
| `image` | Image name and tag | `ghcr.io/immich-app/immich-server:release` |
| `container_name` | Container name | `immich-immich-1` |
| `container` | Full container object | See [Docker Engine API](https://docs.docker.com/engine/api/v1.45/#tag/Container/operation/ContainerList) |
| `system_info` | Docker host info | See [Docker Engine API](https://docs.docker.com/reference/api/engine/v1.45/#tag/System/operation/SystemInfo) |

### Docker Swarm Services

| Variable | Description |
|----------|-------------|
| `service` | Full service object from the Docker Engine API |
| `system_info` | Docker host info |

## Examples

### Simple URL with container name

```yaml
labels:
  kuma.{{container_name}}.http.name: "{{container_name}}"
  kuma.{{container_name}}.http.url: "http://{{container_name}}:8080"
```

### Port from a container label

```yaml
labels:
  kuma.myapp.http.url: "http://{{ container['Labels']['myapp.port'] }}"
```

### Conditional monitor type

```jinja
{% if container['Labels']['myapp.keyword'] %}
  {{ container_name }}.keyword.keyword: {{ container['Labels']['myapp.keyword'] }}
{% endif %}
```
