---
icon: lucide/file-text
---

# Static Monitors

AutoKuma can create monitors from files in addition to reading Docker labels. This is useful for monitors that are not related to a specific container.

## Setup

Place `.json` or `.toml` files in the directory configured by `AUTOKUMA__STATIC_MONITORS`. AutoKuma will pick them up on the next sync.

The default locations are:

| Platform | Path |
|----------|------|
| Linux | `$XDG_CONFIG_HOME/autokuma/static-monitors/` |
| macOS | `$HOME/Library/Application Support/autokuma/static-monitors/` |
| Windows | `%LocalAppData%\autokuma\static-monitors\` |

The filename (without extension) becomes the AutoKuma ID for the monitor.

## File Format

=== "TOML"

    ```toml
    type = "http"
    name = "Example"
    url = "https://example.com"
    interval = 60
    max_retries = 3
    ```

=== "JSON"

    ```json
    {
      "type": "http",
      "name": "Example",
      "url": "https://example.com",
      "interval": 60,
      "max_retries": 3
    }
    ```

See the [entity types reference](../entity-types/index.md) for all available monitor types and their properties.

## Example Files

The [monitors/](https://github.com/BigBoot/AutoKuma/tree/master/monitors) directory in the AutoKuma repository contains example static monitor files you can use as a starting point.

## Symlinks

By default AutoKuma does not follow symlinks when scanning the static monitors directory. Set `AUTOKUMA__FILES__FOLLOW_SYMLINKS=true` to enable symlink traversal.
