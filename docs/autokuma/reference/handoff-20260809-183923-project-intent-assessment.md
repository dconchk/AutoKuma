---
okf_version: "0.2"
type: Audit
title: "Project intent assessment"
description: "The boundary was the standalone `chk/projects/autokuma` repository. All 225 commit subjects, upstream origin and release pivots, both..."
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
    ref: "docs/handoffs/20260809-183923-project-intent-assessment.md"
    resource: "WINTER/Archive/SPACES/chk/projects/autokuma/docs/handoffs/20260809-183923-project-intent-assessment.md"
---

# Project intent assessment

## Assessment basis

The boundary was the standalone `chk/projects/autokuma` repository. All 225 commit subjects, upstream origin and release pivots, both `dconchk` fork commits, current diff, changelog/configuration, relevant Better Auth, synchronization, inventory, tests, and workflows were inspected. Branch `codex/uptime-kuma-3-better-auth` is at `e389999ba2c86f52d06a212c44c8992743f1789a`, two commits ahead of local master and equal to its remote-tracking branch. The worktree has material unstaged changes in `autokuma/src/entity.rs` and `autokuma/src/sync.rs`; no lock or second worktree was found.

## Current state

Committed fork work adds Uptime Kuma 3 Better Auth, process-local cookies, bounded/redacted authentication failure, reusable-token removal, one-shot synchronization, monitor payload compatibility, and combined monitor/tag inventory. Existing tests describe these behaviors but were not run. The dirty tree adds active/paused monitor-state preservation and fail-closed live-state handling without a recorded operator rationale. No fork tag, release, CI result, deployment record, supported target version, or live acceptance exists. Generated target content is historical build residue, not proof.

## Reconstructed intent

The strongest supported fork-specific umbrella is scheduler-friendly AutoKuma reconciliation against Uptime Kuma 3 Better Auth without persisted reusable tokens, repeated authentication, or duplicate inventory sessions. The generic upstream product mission is third-party context, not direct operator intent. Long-term fork maintenance, publication, destructive reconciliation policy, exact target release, and the dirty monitor-state work remain outside this draft.

## Intent confidence

- Confirmed: no direct operator-attributed desired-state statement exists.
- Corroborated: Better Auth compatibility, in-memory session handling, bounded/redacted failure, one-shot mode, and combined monitor/tag inventory across fork commits, code, docs, and tests.
- Inferred: the dirty active/paused preservation policy; excluded from normative outcomes.
- Unknown: fork lifetime, supported release/environment, publication destination, destructive policy, release/live state, and operating mode authority.
- Contradicted: dconchk fork identity versus inherited upstream package, documentation, container, and workflow destinations.

## Contradictions and drift

Nearly all history is upstream-authored. The fork's two commits establish a focused delta, while repository metadata and release automation still point to upstream BigBoot destinations. The changelog calls the fork delta unreleased and no tags exist. Workflows target master or tags, not the custom branch. Recurring mode remains while one-shot was added, with no deployment record choosing an authority. Uncommitted reconciliation safety semantics are substantial but unapproved.

## Unresolved true-intent gaps

1. **High:** Is this a maintained CHK fork, temporary compatibility patch, or upstream contribution? Affects objective, outcomes, scope, and alignment.
2. **High:** Which Uptime Kuma 3 releases, topology, and environment must it support? Affects outcomes, constraints, scope, and verification.
3. **High:** Is preserving live active/paused state the intended policy, including type-changing recreation? Affects objective, outcomes, constraints, and edge cases.
4. **High:** What live verification and release criterion accepts Better Auth and scheduler changes? Affects outcomes, health metrics, and verification.
5. **High:** Where, if anywhere, may fork artifacts be published? Affects outcomes, constraints, scope, and verification.
6. **High:** Which operations may delete or recreate live entities, and what rollback policy applies? Affects constraints, edge cases, health metrics, and verification.
7. **Medium:** Is one-shot or recurring daemon operation authoritative? Affects outcomes, scope, edge cases, and verification.
8. **Medium:** What upstream synchronization policy and compatibility baseline apply? Affects constraints, scope, and alignment.
9. **Medium:** What credential source and secret-management boundary applies in the real deployment? Affects constraints, scope, and edge cases.
10. **Medium:** Does scheduler inventory require only monitors/tags or all managed entity types? Affects outcomes, scope, and verification.

The reversible assumption is a narrow unreleased Better Auth and scheduler delta only, with no publication, deployment, destructive-operation, or dirty monitor-state authority.

## Source coverage

All 225 commit subjects, 48 Markdown files, 56 Rust source files, 65 test annotations, configuration, five workflows, fork commits, current diff, and relevant upstream chronology were inspected. No project-local operator capture, brainstorm, transcript, handoff, decision, plan, milestone, IntentSpec, CI log, deployment record, tag, or live verification exists. Roughly 21 GiB of ignored generated target content was excluded as proof. No unreadable source was found.

## IntentSpec result

Draft IntentSpec: `docs/intents/chk-projects-autokuma-project.intent.md`.

Schema validation: PASS. Semantic validation: PASS.

## Handoff status

Ready for operator review. The draft does not authorize publication, release, deployment, destructive reconciliation, or adoption of the dirty monitor-state policy.
