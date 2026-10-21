---
okf_version: "0.2"
type: System
title: "Autokuma"
description: "Bounded Kuma 3 authentication, reconciliation, inventory and approved ownership adoption."
status: active
concepts:
  - autokuma
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:21+00:00"
updated: "2026-10-21T05:39:21+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-21T05:39:21+00:00"
stale_after: "2027-04-19"
tags:
  - autokuma
  - overview
  - system
relationships:
  - type: depends_on
    target: "../intent.md"
sources:
  - kind: repo_path
    ref: "docs/intent.md"
    resource: "docs/intent.md"
---

# AutoKuma

Rust workspace containing AutoKuma reconciliation, kuma-client and kuma-cli. This CHK fork adds Kuma 3 Better Auth, one-shot operation, combined inventory, observed-state preservation and bounded circuit ownership adoption. Live authority and release compatibility remain separately gated.

Native Cargo workspace and documentation site are under `src/`; run Cargo and Zensical from that directory. Original site docs are retained under `src/docs/`.
