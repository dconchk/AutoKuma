---
okf_version: "0.2"
type: Intent
title: "CHK AutoKuma fork"
description: "Bounded Kuma 3 authentication, reconciliation, inventory and approved ownership adoption."
status: approved
concepts:
  - autokuma
tags:
  - autokuma
  - intent
  - governance
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
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:20+00:00"
updated: "2026-10-21T05:39:20+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-21T05:39:20+00:00"
  - by: human:dcon
    at: "2026-10-21T05:39:20+00:00"
stale_after: "2027-04-19"
sources:
  - kind: repo_path
    ref: "src/CHANGELOG.md"
    resource: "src/CHANGELOG.md"
  - kind: repo_path
    ref: "src/autokuma/src/entity.rs"
    resource: "src/autokuma/src/entity.rs"
  - kind: repo_path
    ref: "src/autokuma/src/state_tool.rs"
    resource: "src/autokuma/src/state_tool.rs"
  - kind: repo_path
    ref: "src/kuma-client/src/better_auth.rs"
    resource: "src/kuma-client/src/better_auth.rs"
  - kind: repo_path
    ref: "src/kuma-cli/src/inventory.rs"
    resource: "src/kuma-cli/src/inventory.rs"
  - kind: repo_path
    ref: "docs/autokuma/reference/handoff-20260810-015558-project-intent-refinement.md"
    resource: "docs/autokuma/reference/handoff-20260810-015558-project-intent-refinement.md"
id: "INTENT-AUTOKUMA-001"
version: 3
schemaVersion: "1.2"
createdAt: 1786300763000
updatedAt: 1792560071961
context:
  project: "autokuma"
  domain: "software-engineering"
  target: "workspace"
objective: "Maintain a CHK AutoKuma fork for bounded Uptime Kuma 3 reconciliation, inventory, observed-state preservation, and explicitly approved circuit ownership adoption without persisted reusable authentication tokens."
userGoal: "Reconcile and inspect authorized Kuma 3 monitor state through schedulable operations while preserving live state and preventing unapproved identity, deletion, publication, or authentication behavior."
problemStatement: "Autokuma requires a unified, verifiable intent contract."
successHeadline: "Maintain a CHK AutoKuma fork for bounded Uptime Kuma 3 reconciliation, inventory, observed-state preservation, and explicitly approved circuit ownership adoption without persisted reusable authentication tokens."
outcomes:
  - "[OUT-01] Valid credentials and any required one-time code establish and validate a process-local Better Auth session before Socket.IO use."
  - "[OUT-02] Authentication failures terminate after a bounded redacted sequence without persisting or disclosing credentials, codes, or session cookies."
  - "[OUT-03] One-shot mode performs one reconciliation, shuts down sources, and returns process success or failure."
  - "[OUT-04] One authenticated CLI session emits combined monitor and tag inventory."
  - "[OUT-05] Existing-monitor update or type recreation preserves the observed live active or paused state unless an approved state-authority contract explicitly supersedes it."
  - "[OUT-06] An approved circuit adoption plan atomically rewrites only validated local monitor/tag ownership IDs and confirms exact readback convergence."
  - "[OUT-07] Given valid credentials and an optional required one-time code, the client establishes and validates an Uptime Kuma 3 Better Auth session before opening Socket.IO while keeping the session cookie process-local."
  - "[OUT-08] Missing or invalid authentication material and rejected sessions terminate after the bounded sign-in sequence without disclosing supplied credentials, one-time codes, or session-cookie values."
  - "[OUT-09] With one-shot mode enabled, AutoKuma performs one reconciliation, shuts down its sources, and returns success or failure through the process result."
  - "[OUT-10] The CLI inventory command emits monitors and tags obtained through one connected client session."
constraints:
  - "[CON-01] Keep reusable JWT login and token persistence removed; keep Better Auth session cookies in process memory."
  - "[CON-02] Bound one sign-in sequence and treat authentication rejection as terminal for the process."
  - "[CON-03] Require an authoritative approved source for every adopted live ID; structural validity alone is insufficient."
  - "[CON-04] Require preflight inventory, database/config backup, bounded target plan, rollback procedure, and post-operation live readback before adoption or destructive recreation."
  - "[CON-05] Do not publish or release through upstream/fork destinations until ownership and destination authority are explicit."
  - "[CON-06] Do not claim compatibility beyond the exact recorded Kuma 3 versions, topology, authentication settings, and environment evidence."
  - "[CON-07] Reusable JWT authentication-token flags, login, and persistence remain removed."
  - "[CON-08] Better Auth session cookies remain in process memory and are not written to configuration or disk."
  - "[CON-09] One sign-in call performs at most one username and password request and, when required, one one-time-code request."
  - "[CON-10] Authentication rejection is terminal for the synchronization process rather than periodically retried."
  - "[CON-11] Combined inventory means monitors and tags only; broader entity coverage is not implied."
  - "[CON-12] Approval of this workspace intent grants no live adoption, destructive reconciliation, publication or release authority."
verification:
  - "[VER-01] Exercise Better Auth success, two-factor, missing-code, invalid-credential, rejected-session, bounded-attempt, and redaction cases."
  - "[VER-02] Exercise one-shot process status and combined monitor/tag inventory through one session."
  - "[VER-03] Test observed-state preservation for update and recreation, including unavailable-state failure."
  - "[VER-04] Validate adoption preflight, authoritative ID approval, atomicity, exact readback, backup, rollback, and unrelated-ID preservation."
  - "[VER-05] Record exact Kuma version/topology and target-environment authentication, scheduler, reconciliation, publication, and production evidence before release or live-verified status."
  - "[VER-06] Exercise the existing Better Auth integration cases for valid two-factor authentication, missing code, invalid credentials, rejected sessions, bounded attempts, and redaction."
  - "[VER-07] Exercise the existing unit coverage for one-shot configuration, terminal authentication classification, reusable-token flag removal, and inventory command parsing."
  - "[VER-08] Do not claim release or live compatibility until the target Uptime Kuma 3 release and authorized environment are recorded and verified."
healthMetrics:
  - "[HM-01] At most one username/password request and one required one-time-code request per sign-in call."
  - "[HM-02] No persisted reusable authentication tokens or session cookies."
  - "[HM-03] Exact monitor and owner-tag readback convergence for every approved adoption."
edgeCases:
  - id: EDGE-01
    scenario: "A two-factor challenge occurs without a one-time code or credentials/session are rejected."
    expectedBehavior: "Stop after the bounded sequence and report only redacted failure."
  - id: EDGE-02
    scenario: "Existing live active or paused state cannot be read before update or recreation."
    expectedBehavior: "Fail closed rather than guessing desired state."
  - id: EDGE-03
    scenario: "An adoption plan is structurally valid but its live IDs lack authoritative approval."
    expectedBehavior: "Refuse adoption before database mutation."
  - id: EDGE-04
    scenario: "Adoption or destructive reconciliation does not match exact readback."
    expectedBehavior: "Stop, preserve evidence, and execute the approved rollback path."
  - id: EDGE-05
    scenario: "A two-factor challenge occurs but no one-time code is available."
    expectedBehavior: "Stop without retrying and report a redacted code-required error."
  - id: EDGE-06
    scenario: "Username or password authentication is rejected."
    expectedBehavior: "Stop after one sign-in attempt and do not expose credential values."
  - id: EDGE-07
    scenario: "A newly issued session fails validation."
    expectedBehavior: "Stop after the bounded sign-in sequence and redact session and credential values."
  - id: EDGE-08
    scenario: "Recurring synchronization encounters an authentication rejection."
    expectedBehavior: "Shut down sources and return failure rather than periodically retrying authentication."
  - id: EDGE-09
    scenario: "One-shot reconciliation fails."
    expectedBehavior: "Shut down sources and propagate process failure."
failureModes:
  - id: FAIL-01
    trigger: "A two-factor challenge occurs without a one-time code or credentials/session are rejected."
    mitigation: "Stop after the bounded sequence and report only redacted failure."
  - id: FAIL-02
    trigger: "Existing live active or paused state cannot be read before update or recreation."
    mitigation: "Fail closed rather than guessing desired state."
  - id: FAIL-03
    trigger: "An adoption plan is structurally valid but its live IDs lack authoritative approval."
    mitigation: "Refuse adoption before database mutation."
  - id: FAIL-04
    trigger: "Adoption or destructive reconciliation does not match exact readback."
    mitigation: "Stop, preserve evidence, and execute the approved rollback path."
  - id: FAIL-05
    trigger: "A two-factor challenge occurs but no one-time code is available."
    mitigation: "Stop without retrying and report a redacted code-required error."
  - id: FAIL-06
    trigger: "Username or password authentication is rejected."
    mitigation: "Stop after one sign-in attempt and do not expose credential values."
  - id: FAIL-07
    trigger: "A newly issued session fails validation."
    mitigation: "Stop after the bounded sign-in sequence and redact session and credential values."
  - id: FAIL-08
    trigger: "Recurring synchronization encounters an authentication rejection."
    mitigation: "Shut down sources and return failure rather than periodically retrying authentication."
  - id: FAIL-09
    trigger: "One-shot reconciliation fails."
    mitigation: "Shut down sources and propagate process failure."
nonGoals:
  - "Reusable JWT-token login and persistence."
  - "Reusable JWT persistence, automatic adoption from untrusted IDs, unapproved destructive reconciliation, unsupported Kuma versions/topologies, and unauthorized upstream or fork publication."
  - "Blanket upstream-feature maintenance, unspecified publication destinations or unverified production topology."
assumptions:
  - id: A-01
    statement: "Repository source artifacts accurately reflect intended system behavior."
    validationMethod: "Cross-check against repository scripts and operator review."
openQuestions: []
evidence:
  - id: EVID-01
    type: observation
    source: "docs/autokuma/reference/handoff-20260810-015558-project-intent-refinement.md"
    excerpt: "Second-pass contract combines bounded Better Auth, one-shot inventory, observed-state preservation and explicitly approved ownership adoption; live authority, backup, rollback and release acceptance remain gated."
    authority: governing
    anchors:
      - "objective"
      - "userGoal"
      - "outcome:0"
      - "outcome:1"
      - "outcome:2"
      - "outcome:3"
      - "outcome:4"
      - "outcome:5"
      - "outcome:6"
      - "outcome:7"
      - "outcome:8"
      - "outcome:9"
      - "constraint:0"
      - "constraint:1"
      - "constraint:2"
      - "constraint:3"
      - "constraint:4"
      - "constraint:5"
      - "constraint:6"
      - "constraint:7"
      - "constraint:8"
      - "constraint:9"
      - "constraint:10"
      - "constraint:11"
      - "edgeCase:0"
      - "edgeCase:1"
      - "edgeCase:2"
      - "edgeCase:3"
      - "edgeCase:4"
      - "edgeCase:5"
      - "edgeCase:6"
      - "edgeCase:7"
      - "edgeCase:8"
      - "healthMetric:0"
      - "healthMetric:1"
      - "healthMetric:2"
---

# CHK AutoKuma fork

## Objective

Maintain a CHK AutoKuma fork for bounded Uptime Kuma 3 reconciliation, inventory, observed-state preservation, and explicitly approved circuit ownership adoption without persisted reusable authentication tokens.

## Problem Statement

Autokuma requires a unified, verifiable intent contract.

## User Goal & Success Headline

- **User Goal**: Reconcile and inspect authorized Kuma 3 monitor state through schedulable operations while preserving live state and preventing unapproved identity, deletion, publication, or authentication behavior.
- **Success Headline**: Maintain a CHK AutoKuma fork for bounded Uptime Kuma 3 reconciliation, inventory, observed-state preservation, and explicitly approved circuit ownership adoption without persisted reusable authentication tokens.

## Outcomes

- [OUT-01] Valid credentials and any required one-time code establish and validate a process-local Better Auth session before Socket.IO use.
- [OUT-02] Authentication failures terminate after a bounded redacted sequence without persisting or disclosing credentials, codes, or session cookies.
- [OUT-03] One-shot mode performs one reconciliation, shuts down sources, and returns process success or failure.
- [OUT-04] One authenticated CLI session emits combined monitor and tag inventory.
- [OUT-05] Existing-monitor update or type recreation preserves the observed live active or paused state unless an approved state-authority contract explicitly supersedes it.
- [OUT-06] An approved circuit adoption plan atomically rewrites only validated local monitor/tag ownership IDs and confirms exact readback convergence.
- [OUT-07] Given valid credentials and an optional required one-time code, the client establishes and validates an Uptime Kuma 3 Better Auth session before opening Socket.IO while keeping the session cookie process-local.
- [OUT-08] Missing or invalid authentication material and rejected sessions terminate after the bounded sign-in sequence without disclosing supplied credentials, one-time codes, or session-cookie values.
- [OUT-09] With one-shot mode enabled, AutoKuma performs one reconciliation, shuts down its sources, and returns success or failure through the process result.
- [OUT-10] The CLI inventory command emits monitors and tags obtained through one connected client session.

## Constraints

- [CON-01] Keep reusable JWT login and token persistence removed; keep Better Auth session cookies in process memory.
- [CON-02] Bound one sign-in sequence and treat authentication rejection as terminal for the process.
- [CON-03] Require an authoritative approved source for every adopted live ID; structural validity alone is insufficient.
- [CON-04] Require preflight inventory, database/config backup, bounded target plan, rollback procedure, and post-operation live readback before adoption or destructive recreation.
- [CON-05] Do not publish or release through upstream/fork destinations until ownership and destination authority are explicit.
- [CON-06] Do not claim compatibility beyond the exact recorded Kuma 3 versions, topology, authentication settings, and environment evidence.
- [CON-07] Reusable JWT authentication-token flags, login, and persistence remain removed.
- [CON-08] Better Auth session cookies remain in process memory and are not written to configuration or disk.
- [CON-09] One sign-in call performs at most one username and password request and, when required, one one-time-code request.
- [CON-10] Authentication rejection is terminal for the synchronization process rather than periodically retried.
- [CON-11] Combined inventory means monitors and tags only; broader entity coverage is not implied.
- [CON-12] Approval of this workspace intent grants no live adoption, destructive reconciliation, publication or release authority.

## Verification

- [VER-01] Exercise Better Auth success, two-factor, missing-code, invalid-credential, rejected-session, bounded-attempt, and redaction cases.
- [VER-02] Exercise one-shot process status and combined monitor/tag inventory through one session.
- [VER-03] Test observed-state preservation for update and recreation, including unavailable-state failure.
- [VER-04] Validate adoption preflight, authoritative ID approval, atomicity, exact readback, backup, rollback, and unrelated-ID preservation.
- [VER-05] Record exact Kuma version/topology and target-environment authentication, scheduler, reconciliation, publication, and production evidence before release or live-verified status.
- [VER-06] Exercise the existing Better Auth integration cases for valid two-factor authentication, missing code, invalid credentials, rejected sessions, bounded attempts, and redaction.
- [VER-07] Exercise the existing unit coverage for one-shot configuration, terminal authentication classification, reusable-token flag removal, and inventory command parsing.
- [VER-08] Do not claim release or live compatibility until the target Uptime Kuma 3 release and authorized environment are recorded and verified.

## Health Metrics

- [HM-01] At most one username/password request and one required one-time-code request per sign-in call.
- [HM-02] No persisted reusable authentication tokens or session cookies.
- [HM-03] Exact monitor and owner-tag readback convergence for every approved adoption.

## Edge Case Matrix

- **EDGE-01** — *A two-factor challenge occurs without a one-time code or credentials/session are rejected.*: Stop after the bounded sequence and report only redacted failure.
- **EDGE-02** — *Existing live active or paused state cannot be read before update or recreation.*: Fail closed rather than guessing desired state.
- **EDGE-03** — *An adoption plan is structurally valid but its live IDs lack authoritative approval.*: Refuse adoption before database mutation.
- **EDGE-04** — *Adoption or destructive reconciliation does not match exact readback.*: Stop, preserve evidence, and execute the approved rollback path.
- **EDGE-05** — *A two-factor challenge occurs but no one-time code is available.*: Stop without retrying and report a redacted code-required error.
- **EDGE-06** — *Username or password authentication is rejected.*: Stop after one sign-in attempt and do not expose credential values.
- **EDGE-07** — *A newly issued session fails validation.*: Stop after the bounded sign-in sequence and redact session and credential values.
- **EDGE-08** — *Recurring synchronization encounters an authentication rejection.*: Shut down sources and return failure rather than periodically retrying authentication.
- **EDGE-09** — *One-shot reconciliation fails.*: Shut down sources and propagate process failure.

## Failure Modes

- **FAIL-01** — *A two-factor challenge occurs without a one-time code or credentials/session are rejected.*: Stop after the bounded sequence and report only redacted failure.
- **FAIL-02** — *Existing live active or paused state cannot be read before update or recreation.*: Fail closed rather than guessing desired state.
- **FAIL-03** — *An adoption plan is structurally valid but its live IDs lack authoritative approval.*: Refuse adoption before database mutation.
- **FAIL-04** — *Adoption or destructive reconciliation does not match exact readback.*: Stop, preserve evidence, and execute the approved rollback path.
- **FAIL-05** — *A two-factor challenge occurs but no one-time code is available.*: Stop without retrying and report a redacted code-required error.
- **FAIL-06** — *Username or password authentication is rejected.*: Stop after one sign-in attempt and do not expose credential values.
- **FAIL-07** — *A newly issued session fails validation.*: Stop after the bounded sign-in sequence and redact session and credential values.
- **FAIL-08** — *Recurring synchronization encounters an authentication rejection.*: Shut down sources and return failure rather than periodically retrying authentication.
- **FAIL-09** — *One-shot reconciliation fails.*: Shut down sources and propagate process failure.

## Non-Goals

- Reusable JWT-token login and persistence.
- Reusable JWT persistence, automatic adoption from untrusted IDs, unapproved destructive reconciliation, unsupported Kuma versions/topologies, and unauthorized upstream or fork publication.
- Blanket upstream-feature maintenance, unspecified publication destinations or unverified production topology.

## Assumptions

- **A-01**: Repository source artifacts accurately reflect intended system behavior. *(Validation: Cross-check against repository scripts and operator review.)*

## Evidence Ledger

- **EVID-01** (`docs/autokuma/reference/handoff-20260810-015558-project-intent-refinement.md`, `observation`, `governing`): Second-pass contract combines bounded Better Auth, one-shot inventory, observed-state preservation and explicitly approved ownership adoption; live authority, backup, rollback and release acceptance remain gated.

## Approval History

- v1 — Draft; no approval recorded (date not recorded).
- v2 — Refined draft; no approval recorded (date not recorded).
- v3 — Approved by Dcon on 2026-10-21 (version 3; operator approval recorded at --apply)
