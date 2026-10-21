---
okf_version: "0.2"
type: Reference
title: "Kuma Client"
description: "Add `kuma-client` to your `Cargo.toml`:"
status: active
concepts:
  - autokuma
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:20+00:00"
updated: "2026-10-21T05:39:20+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-21T05:39:20+00:00"
stale_after: "2027-04-19"
tags:
  - autokuma
  - reference
sources:
  - kind: archive
    ref: "docs/kuma-client.md"
    resource: "WINTER/Archive/SPACES/chk/projects/autokuma/docs/kuma-client.md"
resource: "../../intent.md"
---

# Kuma Client

`kuma-client` is a Rust library for interacting with the Uptime Kuma SocketIO API. It is the foundation that both AutoKuma and Kuma CLI are built on.

## Installation

Add `kuma-client` to your `Cargo.toml`:

```toml
[dependencies]
kuma-client = { git = "https://github.com/BigBoot/AutoKuma.git" }
```

Or find the latest published version on [crates.io](https://crates.io/crates/kuma-client).

## Resources

- [API Documentation on docs.rs](https://docs.rs/kuma-client/latest/kuma_client/)
- [Examples on GitHub](https://github.com/BigBoot/AutoKuma/tree/master/kuma-client/examples)

## Quick Example

```rust
use kuma_client::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::connect(Config {
        url: "http://localhost:3001".parse()?,
        username: Some("admin".into()),
        password: Some("secret".into()),
        ..Default::default()
    })
    .await?;

    let monitors = client.get_monitors().await?;
    for monitor in monitors.values() {
        println!("{}: {}", monitor.common().id().unwrap_or(0), monitor.common().name().as_deref().unwrap_or(""));
    }

    Ok(())
}
```
