---
icon: lucide/clipboard-list
---

# Snippets

Snippets let you define reusable monitor templates. A snippet expands to one or more monitor definitions when referenced from a container label.

!!! tip "Try it in the browser"
    The [AutoKuma Playground](https://autokuma-playground.bigboot.dev/) lets you test templates and snippets interactively.

## Defining Snippets

Snippets are defined in the configuration. Each snippet has a name and a body written in [Tera](https://keats.github.io/tera/). Arguments passed when the snippet is invoked are available as `args[0]`, `args[1]`, etc.

=== "Environment variable"

    ```yaml
    AUTOKUMA__SNIPPETS__WEB: |-
      {{ container_name }}_http.http.name: {{ container_name }} HTTP
      {{ container_name }}_http.http.url: https://{{ args[0] }}:{{ args[1] }}
      {{ container_name }}_docker.docker.name: {{ container_name }} Docker
      {{ container_name }}_docker.docker.docker_container: {{ container_name }}
    ```

=== "TOML config"

    ```toml
    [snippets]
    web = '''
      {{ container_name }}_http.http.name: {{ container_name }} HTTP
      {{ container_name }}_http.http.url: https://{{ args[0] }}:{{ args[1] }}
      {{ container_name }}_docker.docker.name: {{ container_name }} Docker
      {{ container_name }}_docker.docker.docker_container: {{ container_name }}
    '''
    ```

## Using Snippets

To invoke a snippet from a container label, use the format:

```
<prefix>.__<snippet>: <arguments>
```

For example, to use the `web` snippet defined above:

```yaml
labels:
  kuma.__web: '"example.com", 443'
```

The arguments string is parsed as a Tera expression, so you can pass strings, numbers, or JSON objects.

## Advanced Example

Snippets support the full power of Tera. Here is an example that generates different monitor types depending on the arguments:

```jinja
{# Assign the first snippet arg to a variable for easier access #}
{% set args = args[0] %}

{# Generate an ID by slugifying the name argument #}
{% set id = args.name | slugify %}

{# Use keyword monitor if a keyword is specified, otherwise use http #}
{% if args.keyword %}
  {% set type = "keyword" %}
{% else %}
  {% set type = "http" %}
{% endif %}

{{ id }}-group.group.name: {{ args.name }}
{{ id }}-http.{{ type }}.name: {{ args.name }} (HTTP)
{{ id }}-http.{{ type }}.parent_name: {{ id }}-group
{{ id }}-http.{{ type }}.url: {{ args.url }}
{% if args.keyword %}
  {{ id }}-http.{{ type }}.keyword: {{ args.keyword }}
{% endif %}
{{ id }}-container.docker.name: {{ args.name }} (Container)
{{ id }}-container.docker.parent_name: {{ id }}-group
```

Invoking it:

```yaml
# Basic HTTP monitor
labels:
  kuma.__web: '{ "name": "Example HTTP", "url": "https://example.com" }'

# Keyword monitor
labels:
  kuma.__web: '{ "name": "Example", "url": "https://example.com", "keyword": "Example Domain" }'
```

## !Snippets

Snippet names starting with `!` match existing labels from other tools without requiring the `kuma.__` prefix. They always receive a single string argument containing the label value.

This is useful for reusing labels from tools like Traefik:

=== "TOML config"

    ```toml
    [snippets]
    "!traefik.enable" = '''
    {% if args[0] == "true" %}
      {{ container_name }}_http.http.name: {{ container_name }}
      {{ container_name }}_http.http.url: https://{{ container_name }}.example.com
    {% endif %}
    '''
    ```

=== "Environment variables"

    Because `!` and `.` are not valid in environment variable names, `!Snippets` use a two-part `KEY`/`VALUE` form. The numeric index is just a grouping key:

    ```yaml
    AUTOKUMA__SNIPPETS__0__KEY: "!traefik.enable"
    AUTOKUMA__SNIPPETS__0__VALUE: |-
      {% if args[0] == "true" %}
        {{ container_name }}_http.http.name: {{ container_name }}
        {{ container_name }}_http.http.url: https://{{ container_name }}.example.com
      {% endif %}
    ```

### Full Traefik Example

```jinja
{% if args[0] == "true" %}
  {% set traefik_service = container_name %}
  {% set domain = container_name + ".example.com" %}
  {% set port = container["Labels"]["traefik.http.services." + traefik_service + ".loadbalancer.server.port"] %}

  {{ container_name }}_http.http.name: {{ container_name }}
  {{ container_name }}_http.http.url: https://{{ domain }}:{{ port }}
{% endif %}
```
