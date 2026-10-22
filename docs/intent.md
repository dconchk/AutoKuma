---
okf_version: "0.2"
type: Intent
title: "CHK AutoKuma fork for Uptime Kuma 3"
description: "CHK's fork of AutoKuma (Rust): label- and file-driven Uptime Kuma monitor reconciliation plus kuma-cli, extended for Uptime Kuma 3 Better Auth, scheduler-friendly one-shot runs, combined monitor/tag inventory, live pause-state preservation and an approved, atomic circuit-ownership adoption tool."
status: approved
concepts:
  - autokuma
tags:
  - autokuma
  - intent
  - uptime-kuma
  - monitoring
  - rust
relationships:
  - type: depends_on
    target: "constraints/autonomy.md"
  - type: depends_on
    target: "constraints/agent-operating-rules.md"
  - type: depends_on
    target: "conventions/coding-standards.md"
  - type: depends_on
    target: "conventions/codebase-design.md"
  - type: depends_on
    target: "policies/review-rubric.md"
  - type: related_to
    target: "autokuma/autokuma.md"
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:20+00:00"
updated: "2026-10-22T00:59:34-05:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-22T00:59:34-05:00"
  - by: human:dcon
    at: "2026-10-22T00:59:34-05:00"
stale_after: "2027-04-19"
sources:
  - kind: repo_path
    ref: "src/autokuma/Cargo.toml"
    resource: "src/autokuma/Cargo.toml"
  - kind: repo_path
    ref: "src/Cargo.toml"
    resource: "src/Cargo.toml"
  - kind: repo_path
    ref: "src/CHANGELOG.md"
    resource: "src/CHANGELOG.md"
  - kind: repo_path
    ref: "src/kuma-client/src/better_auth.rs"
    resource: "src/kuma-client/src/better_auth.rs"
  - kind: repo_path
    ref: "src/kuma-client/tests/better_auth.rs"
    resource: "src/kuma-client/tests/better_auth.rs"
  - kind: repo_path
    ref: "src/autokuma/src/sync.rs"
    resource: "src/autokuma/src/sync.rs"
  - kind: repo_path
    ref: "src/autokuma/src/entity.rs"
    resource: "src/autokuma/src/entity.rs"
  - kind: repo_path
    ref: "src/autokuma/src/app_state.rs"
    resource: "src/autokuma/src/app_state.rs"
  - kind: repo_path
    ref: "src/autokuma/src/state_tool.rs"
    resource: "src/autokuma/src/state_tool.rs"
  - kind: repo_path
    ref: "src/autokuma/src/config.rs"
    resource: "src/autokuma/src/config.rs"
  - kind: repo_path
    ref: "src/kuma-cli/src/cli.rs"
    resource: "src/kuma-cli/src/cli.rs"
  - kind: repo_path
    ref: "src/kuma-cli/src/inventory.rs"
    resource: "src/kuma-cli/src/inventory.rs"
  - kind: repo_path
    ref: "docs/autokuma/reference/kuma-cli.md"
    resource: "docs/autokuma/reference/kuma-cli.md"
  - kind: repo_path
    ref: "docs/autokuma/reference/kuma-client.md"
    resource: "docs/autokuma/reference/kuma-client.md"
id: "INTENT-AUTOKUMA-001"
version: 4
schemaVersion: "1.2"
createdAt: 1786300763000
updatedAt: 1792648774819
context:
  project: "autokuma"
  domain: "msp-monitoring-automation"
  target: "workspace"
objective: "Maintain CHK's AutoKuma fork so CHK can declare Uptime Kuma 3 monitors from Docker labels, static files and Kubernetes resources, reconcile them on a schedule with process-local Better Auth sessions and bounded, redacted authentication failure, inventory monitors and tags in one session, preserve each existing monitor's live active or paused state, and repoint local ownership records to approved circuit monitor IDs atomically with exact readback."
userGoal: "A CHK engineer runs AutoKuma and kuma-cli against an authorized Uptime Kuma 3 instance from a scheduler, gets monitors created and updated from declared configuration without losing operator pauses or storing reusable tokens, and can inventory or adopt circuit monitor ownership safely when migrating monitors between AutoKuma state stores."
problemStatement: "Upstream AutoKuma targets Uptime Kuma 1/2 token login, keeps a long-running connection, and overwrites an existing monitor's active flag from desired configuration. Uptime Kuma 3 replaced login with Better Auth sessions and two-factor codes, scheduled reconciliation needs a single run that reports success or failure, repeated logins hit authentication rate limits, and re-applying configuration silently un-pauses monitors operators had paused. CHK also needs to move its circuit monitors (primary and backup per circuit) under AutoKuma ownership without duplicating or deleting live monitors."
successHeadline: "Scheduled AutoKuma runs authenticate once per process against Uptime Kuma 3, never persist credentials or cookies, never flip a live monitor's pause state, and adopt circuit ownership only from an approved plan that reads back exactly."
outcomes:
  - "[OUT-01] Reconciliation: the autokuma binary creates, updates and deletes Uptime Kuma monitors, tags, notifications and other entities declared by Docker container/service labels, static JSON/TOML monitor files and Kubernetes custom resources, tracking ownership in its persistent sled database and exposing /health and /metrics."
  - "[OUT-02] Better Auth session: kuma-client signs in through api/auth/sign-in/username, verifies a TOTP code through api/auth/two-factor/verify-totp when a two-factor redirect is returned, and validates the session at api/auth/get-session before opening Socket.IO, keeping the session cookie in a process-local cookie jar."
  - "[OUT-03] Bounded, redacted authentication failure: missing or invalid credentials, a missing one-time code and a rejected new session terminate the sign-in call after one sequence with errors that never contain credentials, codes or cookie values."
  - "[OUT-04] One-shot mode: with run_once (AUTOKUMA__RUN_ONCE=true) AutoKuma performs one synchronization, shuts down its sources and returns the result as the process exit status."
  - "[OUT-05] Combined inventory: `kuma inventory` emits monitors and tags fetched through one authenticated client session, keeping scheduled reconciliation below Better Auth rate limits."
  - "[OUT-06] Live state preservation: updating an existing monitor or recreating it because its type changed keeps the observed live active/paused state; new monitors start active unless creation is explicitly paused."
  - "[OUT-07] Circuit ownership adoption: the autokuma-state binary inventories the AutoKuma ownership database (`inventory --data-path`) and, from an autokuma.circuit-ownership-adoption/v1 plan (`adopt --data-path --plan`), atomically rewrites only the listed circuit_v1_<site>_<circuit>_(primary|backup) monitor IDs and the dcon-circuit-owner tag ID, then confirms exact readback convergence."
  - "[OUT-08] Uptime Kuma 3 payload compatibility: monitor inventory accepts numeric proxy and remote-browser IDs and reports monitor-list schema failures without panicking or echoing monitor values."
constraints:
  - "[CON-01] Reusable JWT token login, token flags and token persistence stay removed from AutoKuma and kuma-cli; Better Auth session cookies live only in process memory and are never written to configuration or disk, and credential types do not implement Debug."
  - "[CON-02] One sign-in call makes at most one username/password request and, when required, one one-time-code request; authentication rejection is terminal for the synchronization process rather than periodically retried."
  - "[CON-03] Adoption requires an authoritative, approved source for every adopted live ID: structural validity of the plan (schema, owner tag name, identity grammar, positive IDs, expected-ID match) is necessary but not sufficient."
  - "[CON-04] Live adoption or destructive recreation requires a preflight inventory, a backup of the AutoKuma database and configuration, a bounded target plan, a rollback procedure and post-operation live readback."
  - "[CON-05] Do not publish or release through the upstream or any fork destination (GHCR images, crates, GitHub releases, docs or playground Pages) until fork ownership and destination authority are explicit; the inherited workflows still point at the upstream project."
  - "[CON-06] Do not claim compatibility beyond the exact recorded Uptime Kuma 3 version, topology, authentication settings and environment evidence."
  - "[CON-07] Combined inventory means monitors and tags only; broader entity coverage is not implied."
  - "[CON-08] Existing-monitor updates fail closed when the live active state cannot be read; desired configuration never silently overrides an observed pause."
  - "[CON-09] This intent grants no live adoption, destructive reconciliation, publication or release authority; each needs a separate explicit approval."
verification:
  - "[VER-01] From src/, run `cargo test -p kuma-client --test better_auth` and confirm the TOTP success, missing-code, invalid-credential and rejected-session cases pass with redacted errors and a single sign-in attempt."
  - "[VER-02] Run `cargo test -p autokuma` and confirm the run_once configuration, monitor active-state preservation (update, recreation, fail-closed and create-paused) and ownership-adoption (full repoint; stale or duplicate plan leaves every mapping unchanged) tests pass."
  - "[VER-03] Run `cargo test -p kuma-cli` and confirm the combined-inventory command parsing and login tests pass."
  - "[VER-04] Before any live adoption, run `autokuma-state inventory --data-path <abs>` against a backed-up database, apply the approved plan with `autokuma-state adopt`, and confirm the result reports verified true with the expected adopted monitor count."
  - "[VER-05] Before claiming release or live compatibility, record the exact Uptime Kuma 3 version, topology, authentication settings, scheduler result and live readback for the target environment."
healthMetrics:
  - "[HM-01] At most one username/password request and one one-time-code request per sign-in call."
  - "[HM-02] Zero persisted reusable authentication tokens or session cookies."
  - "[HM-03] Exact monitor and owner-tag readback convergence for every approved adoption."
  - "[HM-04] Zero live monitors un-paused or paused by reconciliation alone."
edgeCases:
  - id: EDGE-01
    scenario: "A two-factor challenge occurs but no one-time code is configured"
    expectedBehavior: "Stop without retrying and report a redacted code-required error."
  - id: EDGE-02
    scenario: "Username or password authentication is rejected, or a newly issued session fails validation"
    expectedBehavior: "Stop after the single sign-in sequence and expose no credential, code or session values."
  - id: EDGE-03
    scenario: "Recurring synchronization encounters an authentication rejection"
    expectedBehavior: "Shut down sources and return failure rather than periodically retrying authentication."
  - id: EDGE-04
    scenario: "One-shot reconciliation fails"
    expectedBehavior: "Shut down sources and propagate the failure as the process exit status."
  - id: EDGE-05
    scenario: "The live active or paused state of an existing monitor cannot be read before update or recreation"
    expectedBehavior: "Fail closed rather than guessing the desired state."
  - id: EDGE-06
    scenario: "An adoption plan is structurally valid but its live IDs lack authoritative approval, or an expected ID is stale or duplicated"
    expectedBehavior: "Refuse adoption before any database mutation and leave every mapping unchanged."
  - id: EDGE-07
    scenario: "Adoption or destructive reconciliation does not match exact readback"
    expectedBehavior: "Stop, preserve evidence and execute the approved rollback path."
  - id: EDGE-08
    scenario: "A data or plan path is relative, a symlink or not normalized"
    expectedBehavior: "autokuma-state rejects the path before opening the database."
failureModes:
  - id: FAIL-01
    trigger: "Credentials or a session cookie leak into logs, configuration or disk"
    mitigation: "Keep cookies in the process-local jar, keep credential types free of Debug, and redact every authentication error."
  - id: FAIL-02
    trigger: "Scheduled runs hammer Better Auth and trip rate limits"
    mitigation: "Use one session per process, the combined inventory command, and terminal (non-retried) authentication failure."
  - id: FAIL-03
    trigger: "Re-applying configuration un-pauses monitors an operator paused"
    mitigation: "Merge updates and recreations with the observed live active state and fail closed when it is unavailable."
  - id: FAIL-04
    trigger: "Well-formed but wrong IDs are adopted and AutoKuma takes over or deletes the wrong monitors"
    mitigation: "Require approved authoritative IDs, a backup and rollback, atomic sled transactions and exact readback."
  - id: FAIL-05
    trigger: "A fork build is published under the upstream project's registries"
    mitigation: "Keep release and publication gated until fork destination authority is explicit."
nonGoals:
  - "Reusable JWT-token login and persistence."
  - "Automatic adoption from untrusted IDs, unapproved destructive reconciliation, and unsupported Uptime Kuma versions or topologies."
  - "Blanket upstream-feature maintenance, unspecified publication destinations or unverified production topology."
assumptions:
  - id: A-01
    statement: "The target Uptime Kuma 3 instance exposes the Better Auth username sign-in, TOTP verification and get-session endpoints used by kuma-client."
    validationMethod: "Confirm against the target instance's recorded version before a scheduled deployment."
  - id: A-02
    statement: "Whether circuit adoption is permanent fork behavior or a one-time migration tool, and whether recurring or one-shot mode is authoritative in deployment, is still for Dcon to decide."
    validationMethod: "Record the decision as an intent amendment before changing either behavior."
openQuestions: []
evidence:
  - id: EVID-01
    type: observation
    source: "src/autokuma/Cargo.toml"
    excerpt: "Automates the creation of Uptime Kuma monitors based on Docker container labels"
    authority: governing
    anchors:
      - "objective"
      - "userGoal"
      - "outcome:0"
  - id: EVID-02
    type: observation
    source: "src/kuma-client/src/better_auth.rs"
    excerpt: "/// Better Auth HTTP client with a process-local cookie jar."
    authority: governing
    anchors:
      - "outcome:1"
      - "constraint:0"
      - "healthMetric:1"
      - "edgeCase:0"
  - id: EVID-03
    type: observation
    source: "src/kuma-client/src/better_auth.rs"
    excerpt: "Session rejection after that sequence is terminal for"
    authority: governing
    anchors:
      - "outcome:2"
      - "constraint:1"
      - "edgeCase:1"
      - "edgeCase:2"
      - "healthMetric:0"
  - id: EVID-04
    type: observation
    source: "src/CHANGELOG.md"
    excerpt: "Target Uptime Kuma 3 authentication and remove reusable JWT token login and persistence from AutoKuma and Kuma CLI."
    authority: governing
    anchors:
      - "objective"
      - "constraint:0"
      - "constraint:4"
      - "constraint:5"
      - "constraint:8"
      - "outcome:7"
  - id: EVID-05
    type: observation
    source: "src/autokuma/src/sync.rs"
    excerpt: "if self.app_state.config.run_once {"
    authority: governing
    anchors:
      - "outcome:3"
      - "edgeCase:3"
  - id: EVID-06
    type: observation
    source: "src/kuma-cli/src/cli.rs"
    excerpt: "/// Read monitors and tags through one authenticated connection."
    authority: governing
    anchors:
      - "outcome:4"
      - "constraint:6"
  - id: EVID-07
    type: observation
    source: "src/autokuma/src/entity.rs"
    excerpt: "Cannot update existing monitor because its active state is unavailable"
    authority: governing
    anchors:
      - "outcome:5"
      - "constraint:7"
      - "edgeCase:4"
      - "healthMetric:3"
  - id: EVID-08
    type: observation
    source: "src/autokuma/src/state_tool.rs"
    excerpt: "AutoKuma ownership readback did not converge"
    authority: governing
    anchors:
      - "outcome:6"
      - "constraint:2"
      - "constraint:3"
      - "edgeCase:5"
      - "edgeCase:6"
      - "edgeCase:7"
      - "healthMetric:2"
---

# CHK AutoKuma fork for Uptime Kuma 3

## Objective

Maintain CHK's AutoKuma fork so CHK can declare Uptime Kuma 3 monitors from Docker labels, static files and Kubernetes resources, reconcile them on a schedule with process-local Better Auth sessions and bounded, redacted authentication failure, inventory monitors and tags in one session, preserve each existing monitor's live active or paused state, and repoint local ownership records to approved circuit monitor IDs atomically with exact readback.

## Problem Statement

Upstream AutoKuma targets Uptime Kuma 1/2 token login, keeps a long-running connection, and overwrites an existing monitor's active flag from desired configuration. Uptime Kuma 3 replaced login with Better Auth sessions and two-factor codes, scheduled reconciliation needs a single run that reports success or failure, repeated logins hit authentication rate limits, and re-applying configuration silently un-pauses monitors operators had paused. CHK also needs to move its circuit monitors (primary and backup per circuit) under AutoKuma ownership without duplicating or deleting live monitors.

## User Goal & Success Headline

- **User Goal**: A CHK engineer runs AutoKuma and kuma-cli against an authorized Uptime Kuma 3 instance from a scheduler, gets monitors created and updated from declared configuration without losing operator pauses or storing reusable tokens, and can inventory or adopt circuit monitor ownership safely when migrating monitors between AutoKuma state stores.
- **Success Headline**: Scheduled AutoKuma runs authenticate once per process against Uptime Kuma 3, never persist credentials or cookies, never flip a live monitor's pause state, and adopt circuit ownership only from an approved plan that reads back exactly.

## Outcomes

- [OUT-01] Reconciliation: the autokuma binary creates, updates and deletes Uptime Kuma monitors, tags, notifications and other entities declared by Docker container/service labels, static JSON/TOML monitor files and Kubernetes custom resources, tracking ownership in its persistent sled database and exposing /health and /metrics.
- [OUT-02] Better Auth session: kuma-client signs in through api/auth/sign-in/username, verifies a TOTP code through api/auth/two-factor/verify-totp when a two-factor redirect is returned, and validates the session at api/auth/get-session before opening Socket.IO, keeping the session cookie in a process-local cookie jar.
- [OUT-03] Bounded, redacted authentication failure: missing or invalid credentials, a missing one-time code and a rejected new session terminate the sign-in call after one sequence with errors that never contain credentials, codes or cookie values.
- [OUT-04] One-shot mode: with run_once (AUTOKUMA__RUN_ONCE=true) AutoKuma performs one synchronization, shuts down its sources and returns the result as the process exit status.
- [OUT-05] Combined inventory: `kuma inventory` emits monitors and tags fetched through one authenticated client session, keeping scheduled reconciliation below Better Auth rate limits.
- [OUT-06] Live state preservation: updating an existing monitor or recreating it because its type changed keeps the observed live active/paused state; new monitors start active unless creation is explicitly paused.
- [OUT-07] Circuit ownership adoption: the autokuma-state binary inventories the AutoKuma ownership database (`inventory --data-path`) and, from an autokuma.circuit-ownership-adoption/v1 plan (`adopt --data-path --plan`), atomically rewrites only the listed circuit_v1_<site>_<circuit>_(primary|backup) monitor IDs and the dcon-circuit-owner tag ID, then confirms exact readback convergence.
- [OUT-08] Uptime Kuma 3 payload compatibility: monitor inventory accepts numeric proxy and remote-browser IDs and reports monitor-list schema failures without panicking or echoing monitor values.

## Constraints

- [CON-01] Reusable JWT token login, token flags and token persistence stay removed from AutoKuma and kuma-cli; Better Auth session cookies live only in process memory and are never written to configuration or disk, and credential types do not implement Debug.
- [CON-02] One sign-in call makes at most one username/password request and, when required, one one-time-code request; authentication rejection is terminal for the synchronization process rather than periodically retried.
- [CON-03] Adoption requires an authoritative, approved source for every adopted live ID: structural validity of the plan (schema, owner tag name, identity grammar, positive IDs, expected-ID match) is necessary but not sufficient.
- [CON-04] Live adoption or destructive recreation requires a preflight inventory, a backup of the AutoKuma database and configuration, a bounded target plan, a rollback procedure and post-operation live readback.
- [CON-05] Do not publish or release through the upstream or any fork destination (GHCR images, crates, GitHub releases, docs or playground Pages) until fork ownership and destination authority are explicit; the inherited workflows still point at the upstream project.
- [CON-06] Do not claim compatibility beyond the exact recorded Uptime Kuma 3 version, topology, authentication settings and environment evidence.
- [CON-07] Combined inventory means monitors and tags only; broader entity coverage is not implied.
- [CON-08] Existing-monitor updates fail closed when the live active state cannot be read; desired configuration never silently overrides an observed pause.
- [CON-09] This intent grants no live adoption, destructive reconciliation, publication or release authority; each needs a separate explicit approval.

## Verification

- [VER-01] From src/, run `cargo test -p kuma-client --test better_auth` and confirm the TOTP success, missing-code, invalid-credential and rejected-session cases pass with redacted errors and a single sign-in attempt.
- [VER-02] Run `cargo test -p autokuma` and confirm the run_once configuration, monitor active-state preservation (update, recreation, fail-closed and create-paused) and ownership-adoption (full repoint; stale or duplicate plan leaves every mapping unchanged) tests pass.
- [VER-03] Run `cargo test -p kuma-cli` and confirm the combined-inventory command parsing and login tests pass.
- [VER-04] Before any live adoption, run `autokuma-state inventory --data-path <abs>` against a backed-up database, apply the approved plan with `autokuma-state adopt`, and confirm the result reports verified true with the expected adopted monitor count.
- [VER-05] Before claiming release or live compatibility, record the exact Uptime Kuma 3 version, topology, authentication settings, scheduler result and live readback for the target environment.

## Health Metrics

- [HM-01] At most one username/password request and one one-time-code request per sign-in call.
- [HM-02] Zero persisted reusable authentication tokens or session cookies.
- [HM-03] Exact monitor and owner-tag readback convergence for every approved adoption.
- [HM-04] Zero live monitors un-paused or paused by reconciliation alone.

## Edge Case Matrix

- **EDGE-01** — *A two-factor challenge occurs but no one-time code is configured*: Stop without retrying and report a redacted code-required error.
- **EDGE-02** — *Username or password authentication is rejected, or a newly issued session fails validation*: Stop after the single sign-in sequence and expose no credential, code or session values.
- **EDGE-03** — *Recurring synchronization encounters an authentication rejection*: Shut down sources and return failure rather than periodically retrying authentication.
- **EDGE-04** — *One-shot reconciliation fails*: Shut down sources and propagate the failure as the process exit status.
- **EDGE-05** — *The live active or paused state of an existing monitor cannot be read before update or recreation*: Fail closed rather than guessing the desired state.
- **EDGE-06** — *An adoption plan is structurally valid but its live IDs lack authoritative approval, or an expected ID is stale or duplicated*: Refuse adoption before any database mutation and leave every mapping unchanged.
- **EDGE-07** — *Adoption or destructive reconciliation does not match exact readback*: Stop, preserve evidence and execute the approved rollback path.
- **EDGE-08** — *A data or plan path is relative, a symlink or not normalized*: autokuma-state rejects the path before opening the database.

## Failure Modes

- **FAIL-01** — *Credentials or a session cookie leak into logs, configuration or disk*: Keep cookies in the process-local jar, keep credential types free of Debug, and redact every authentication error.
- **FAIL-02** — *Scheduled runs hammer Better Auth and trip rate limits*: Use one session per process, the combined inventory command, and terminal (non-retried) authentication failure.
- **FAIL-03** — *Re-applying configuration un-pauses monitors an operator paused*: Merge updates and recreations with the observed live active state and fail closed when it is unavailable.
- **FAIL-04** — *Well-formed but wrong IDs are adopted and AutoKuma takes over or deletes the wrong monitors*: Require approved authoritative IDs, a backup and rollback, atomic sled transactions and exact readback.
- **FAIL-05** — *A fork build is published under the upstream project's registries*: Keep release and publication gated until fork destination authority is explicit.

## Non-Goals

- Reusable JWT-token login and persistence.
- Automatic adoption from untrusted IDs, unapproved destructive reconciliation, and unsupported Uptime Kuma versions or topologies.
- Blanket upstream-feature maintenance, unspecified publication destinations or unverified production topology.

## Assumptions

- **A-01**: The target Uptime Kuma 3 instance exposes the Better Auth username sign-in, TOTP verification and get-session endpoints used by kuma-client. *(Validation: Confirm against the target instance's recorded version before a scheduled deployment.)*
- **A-02**: Whether circuit adoption is permanent fork behavior or a one-time migration tool, and whether recurring or one-shot mode is authoritative in deployment, is still for Dcon to decide. *(Validation: Record the decision as an intent amendment before changing either behavior.)*

## Evidence Ledger

- **EVID-01** (`src/autokuma/Cargo.toml`, `observation`, `governing`): Automates the creation of Uptime Kuma monitors based on Docker container labels
- **EVID-02** (`src/kuma-client/src/better_auth.rs`, `observation`, `governing`): /// Better Auth HTTP client with a process-local cookie jar.
- **EVID-03** (`src/kuma-client/src/better_auth.rs`, `observation`, `governing`): Session rejection after that sequence is terminal for
- **EVID-04** (`src/CHANGELOG.md`, `observation`, `governing`): Target Uptime Kuma 3 authentication and remove reusable JWT token login and persistence from AutoKuma and Kuma CLI.
- **EVID-05** (`src/autokuma/src/sync.rs`, `observation`, `governing`): `if self.app_state.config.run_once {`
- **EVID-06** (`src/kuma-cli/src/cli.rs`, `observation`, `governing`): /// Read monitors and tags through one authenticated connection.
- **EVID-07** (`src/autokuma/src/entity.rs`, `observation`, `governing`): Cannot update existing monitor because its active state is unavailable
- **EVID-08** (`src/autokuma/src/state_tool.rs`, `observation`, `governing`): AutoKuma ownership readback did not converge

## Approval

Dcon has approved version 4 of this intent on 2026-10-22.

## Approval History

- v1 — Draft; no approval recorded (date not recorded).
- v2 — Refined draft; no approval recorded (date not recorded).
- v3 — Approved by Dcon on 2026-10-21 (version 3; operator approval recorded at --apply)
- v4 — Approved by Dcon on 2026-10-22 (Resynthesized as a solution-focused intent)
