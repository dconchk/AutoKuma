---
okf_version: "0.2"
type: Audit
title: "Project intent refinement — second evidence pass"
description: "A corrected draft IntentSpec remains supportable for the active CHK AutoKuma fork. Current fork intent combines Uptime Kuma 3 Better..."
status: active
concepts:
  - autokuma
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:19+00:00"
updated: "2026-10-21T05:39:19+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-21T05:39:19+00:00"
stale_after: "2027-04-19"
tags:
  - autokuma
  - audit
sources:
  - kind: archive
    ref: "docs/handoffs/20260810-015558-project-intent-refinement.md"
    resource: "WINTER/Archive/SPACES/chk/projects/autokuma/docs/handoffs/20260810-015558-project-intent-refinement.md"
---

# Project intent refinement — second evidence pass

## Disposition

A corrected draft IntentSpec remains supportable for the active CHK AutoKuma fork. Current fork intent combines Uptime Kuma 3 Better Auth, bounded authentication failure, one-shot reconciliation, combined monitor/tag inventory, preservation of observed active/paused state, and a CHK-specific circuit ownership-adoption operation.

The refined candidate is `20260810-015558-autokuma-intent-refined-candidate.intent.md`. It does not authorize live adoption, destructive recreation, publication, or release and explicitly requires authoritative ID plans, backup/rollback, exact Kuma compatibility, and live acceptance.

## Evidence and current state

- The branch `codex/uptime-kuma-3-better-auth` is at `b3e7530`, three CHK commits beyond upstream `master` and one ahead of its recorded remote-tracking feature branch.
- Better Auth, one-shot, inventory, monitor-state preservation, and circuit ownership adoption are committed code with source tests, but no tests were run in this assessment.
- Release workflows build/publish only from `master` or tags (`.github/workflows/docker-build-push.yml:3-11`; `.github/workflows/rust-build-release.yml:3-12`). The changelog remains `Unreleased` and omits later inventory/adoption work (`CHANGELOG.md:8-19`).
- Existing-monitor updates and type-changing recreation preserve observed live active/paused state and fail closed when unavailable (`autokuma/src/entity.rs:560-627,681-779`). This applies generically, not only to adoption.
- Normal type changes can delete/recreate monitors, while the adoption tool atomically rewrites local monitor/tag ownership IDs (`autokuma/src/sync.rs:139-170`; `autokuma/src/app_state.rs:358-469`). No approved live authority, preflight snapshot, or rollback procedure exists.
- The adoption tool accepts externally supplied IDs, validates structure/local expectations, rewrites the database, and reads back convergence (`autokuma/src/state_tool.rs:87-105,143-182`). It cannot determine whether well-formed adopted IDs are the correct live authorities.
- The otherwise generic fork hardcodes a CHK circuit schema, owner tag, and identity grammar (`autokuma/src/state_tool.rs:12-13,87-104`). Permanent-versus-temporary scope is unknown.
- No exact Kuma 3 version/topology, current CI result, target authentication proof, scheduler result, live reconciliation, release artifact, publication authority, or production readback is retained.

## First-pass gap reconciliation

| Prior gap | Second-pass result |
|---|---|
| Fork purpose/lifetime | Narrowed: this is an active CHK fork; ownership, maintenance, upstream, and retirement remain High. |
| Supported Kuma 3 release/topology | Still High; exact compatibility/deployment matrix is absent. |
| Preserve active/paused state | Resolved as committed policy, but source-of-truth scope remains High because desired config no longer controls existing pause state. |
| Live verification/release criterion | Still High; no current target/release/production evidence exists. |
| Publication destination | Still High; upstream metadata/workflows remain while authorized fork destination is undefined. |
| Destructive reconciliation and rollback | Still High; recreation/adoption lack approved live authority and recovery contract. |

## Current true-intent and safety gaps

1. **High — What authoritative live source supplies adopted monitor/tag IDs, who approves each plan, and how are wrong-but-well-formed IDs rejected?**
2. **High — Is CHK circuit adoption permanent supported fork behavior, a temporary migration tool, or code that belongs elsewhere?**
3. **High — Is observed live active/paused state always authoritative for existing monitors, or may approved desired configuration override it?**
4. **High — What fork owner, upstream-sync policy, retirement condition, release destination, and publication prohibition/authority apply?**
5. **High — What preflight snapshot, backup, rollback, exact target-version matrix, and live readback are required before destructive recreation or adoption?**
6. **Medium — Is recurring or one-shot mode authoritative in deployment?**
7. **Medium — Where are inventory/adoption commands, backup expectations, recovery, and CHK operations documented?**

## Supported contract boundary

The fork may establish process-local Better Auth sessions with bounded/redacted failures; run once with process status; inventory monitors/tags through one session; preserve observed state for existing monitors; and perform an explicitly approved, atomic, fail-closed circuit ownership adoption. Live/destructive use requires exact target compatibility, authoritative IDs, preflight, backup, rollback, readback, and publication/ownership decisions.

This is a complete second-pass disposition. Operator input is needed for live authority, state ownership, fork maintenance, and release—not to establish current fork intent.

## Validation

- Root spot-check: PASS for branch/commit topology, workflow triggers, unreleased changelog, hardcoded circuit contract, adoption validation/readback, and untracked assessment-only state.
- Schema validation: PASS against the exact portfolio schema.
- Semantic validation: PASS; committed behavior is distinguished from release/live authority.
