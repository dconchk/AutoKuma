---
okf_version: "0.2"
type: System
title: "AutoKuma Fork — System Architecture"
description: "Crates, binaries, authentication flow, reconciliation and ownership-adoption workflows, and safety boundaries of CHK's Uptime Kuma 3 AutoKuma fork."
status: active
concepts:
  - autokuma
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:21+00:00"
updated: "2026-10-22T01:04:11-05:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-22T01:04:11-05:00"
stale_after: "2027-04-19"
tags:
  - autokuma
  - overview
  - system
  - uptime-kuma
relationships:
  - type: depends_on
    target: "../intent.md"
  - type: related_to
    target: "reference/kuma-cli.md"
  - type: related_to
    target: "reference/kuma-client.md"
sources:
  - kind: repo_path
    ref: "docs/intent.md"
    resource: "docs/intent.md"
  - kind: repo_path
    ref: "src/Cargo.toml"
    resource: "src/Cargo.toml"
  - kind: repo_path
    ref: "src/kuma-client/src/better_auth.rs"
    resource: "src/kuma-client/src/better_auth.rs"
  - kind: repo_path
    ref: "src/autokuma/src/state_tool.rs"
    resource: "src/autokuma/src/state_tool.rs"
---

# AutoKuma Fork — System Architecture

AutoKuma (upstream: BigBoot/AutoKuma, MIT) turns declarations — Docker container and service labels, static JSON/TOML files, Kubernetes custom resources — into Uptime Kuma monitors, tags, notifications and related entities, and keeps them reconciled. CHK maintains a fork that targets **Uptime Kuma 3** and adds what scheduled, CHK-operated monitoring needs. The contract is [the project intent](../intent.md).

## The problem the fork solves

- Uptime Kuma 3 replaced token login with **Better Auth** sessions and two-factor codes; upstream still used reusable JWT tokens.
- CHK runs reconciliation from a **scheduler**, so it needs one run with a meaningful exit code, and one login per process to stay under Better Auth rate limits.
- Re-applying desired configuration used to **un-pause monitors** an operator had paused.
- CHK's **circuit monitors** (a primary and a backup per site circuit) needed to move under AutoKuma ownership without duplicating or deleting live monitors.

## Components

The Rust workspace lives in `src/` (`src/Cargo.toml`, members `autokuma`, `kuma-client`, `kuma-cli`, `autokuma-playground`). Run Cargo from `src/`.

| Component | Path | Role |
| --- | --- | --- |
| `kuma-client` library | `src/kuma-client` | Socket.IO client for Uptime Kuma. Fork adds `better_auth.rs`: sign-in, TOTP verification and session validation with a process-local cookie jar. |
| `autokuma` binary | `src/autokuma/src/main.rs`, `sync.rs` | Reconciler. Sources in `src/autokuma/src/sources` (Docker, file, Kubernetes); ownership state in a sled database (`app_state.rs`, migrations v1–v3); `/health` and `/metrics` on port 8090 by default. |
| `autokuma-state` binary | `src/autokuma/src/state_tool.rs` | CHK-only ownership tool: `inventory` and `adopt` against the AutoKuma database. |
| `crdgen` binary | `src/autokuma/src/crdgen.rs` | Generates the Kubernetes CRDs (`src/autokuma/kubernetes/crds-autokuma.yml`). |
| `kuma` CLI | `src/kuma-cli` | Manage monitors, tags, notifications, maintenance, status pages and Docker hosts; fork adds `kuma inventory`. |
| Playground | `src/autokuma-playground` | Browser playground for label templates (Trunk build). |
| Upstream user docs | `src/docs`, `src/zensical.toml` | Configuration, labels, snippets, templating, static monitors, entity types. |

## Authentication flow (Better Auth)

1. Reuse a still-valid process-local session if `api/auth/get-session` returns a session and user.
2. Otherwise POST `api/auth/sign-in/username` once (`rememberMe` false).
3. If the response asks for two-factor, POST one TOTP code to `api/auth/two-factor/verify-totp`; with no code configured, stop with a redacted code-required error.
4. Validate the new session; on rejection, stop. Only then open Socket.IO, passing the cookie header from memory.

Any failure is terminal for the process: no retry loop, no token or cookie written anywhere, no credential in error text. Reusable JWT token flags and persistence were removed from both `autokuma` and `kuma`.

## Reconciliation workflow

- **Recurring mode** (upstream default): keep the connection alive and sync on an interval.
- **One-shot mode**: `run_once` / `AUTOKUMA__RUN_ONCE=true` runs one sync, shuts down sources and exits with the sync result — the mode for schedulers.
- **State preservation**: on update, and on delete-and-recreate when a monitor's type changes, the merged entity keeps the live `active` value. If the live value cannot be read the update fails closed. New monitors start active unless creation is explicitly paused.
- **Inventory**: `kuma inventory` returns `{monitors, tags}` from one authenticated session.

## Circuit ownership adoption

`autokuma-state` takes only absolute, normalized, non-symlink paths:

- `autokuma-state inventory --data-path <dir>` prints `autokuma.ownership-state-inventory/v1` JSON: sorted monitor identities and tag names with their Uptime Kuma IDs.
- `autokuma-state adopt --data-path <dir> --plan <file>` reads an `autokuma.circuit-ownership-adoption/v1` plan: owner tag `dcon-circuit-owner` with expected and adopted IDs, plus monitors named `circuit_v1_<site>_<circuit>_primary|backup` with optional expected ID and adopted ID. It validates the plan, rewrites monitor and tag IDs in one sled transaction (a stale or duplicate plan aborts with nothing changed), then reads back every mapping and reports `verified: true` only on exact convergence.

The tool cannot tell whether a well-formed ID is the right live monitor. That judgment, plus backup, rollback and live readback, is a human gate.

## Safety boundaries

- No live adoption, destructive recreation, publication or release without a separate explicit approval.
- Release and publication stay gated: the inherited GitHub workflows (`.github/workflows`) build and push to the upstream project's GHCR images, crates, docs and Pages from `master` or tags.
- Compatibility is claimed only for a recorded Uptime Kuma 3 version, topology and authentication setup.
- `kuma inventory` covers monitors and tags only.

## Open decisions for Dcon

1. What authoritative live source supplies adopted IDs, and who approves each plan?
2. Is circuit adoption permanent fork behavior or a one-time migration tool?
3. Is observed live pause state always authoritative, or may approved desired configuration override it?
4. Who owns the fork, what is the upstream-sync policy, and where (if anywhere) may it be released?
5. Is recurring or one-shot mode the deployed mode, and where are CHK runbooks for inventory, adoption, backup and recovery kept?

## Verification

From `src/`: `cargo test -p kuma-client --test better_auth`, `cargo test -p autokuma`, `cargo test -p kuma-cli`. The fork changes are in the three CHK commits on top of upstream 2.1.0-rc.2: Better Auth and one-shot, combined inventory, and state preservation with ownership adoption.

## Where to read next

- [Kuma CLI](reference/kuma-cli.md) — command reference.
- [Kuma Client](reference/kuma-client.md) — library usage.
- [Project intent](../intent.md) — outcomes, constraints and verification.
