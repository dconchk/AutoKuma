---
okf_version: "0.2"
type: Policy
title: "Review rubric"
description: "The three isolated review axes (Standards, Spec, Adversarial), the six-part finding shape, and landing gates."
status: active
owner: dcon
concepts: [autokuma]
tags:
  - governance
  - review-rubric
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:20+00:00"
updated: "2026-10-21T05:39:20+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-21T05:39:20+00:00"
stale_after: "2027-04-19"
---

# Review rubric

A review is three isolated axes, each judged by its own seat: **Standards**, **Spec** and **Adversarial**. A reviewer judges and cites; it never edits, commits or fixes. It may run deterministic read-only checks to falsify a claim, and cites each one it ran.

## The axes

- **Spec** judges the frozen candidate SHA against its merge base (the three-dot diff) for complete requested behaviour and scope, against the ticket, its specification and the approved intent. A seam the diff creates or reorders is judged here whether or not the ticket names it.
- **Standards** judges the diff against the project's documented standards ([Coding standards](../conventions/coding-standards.md)) and the inherited baseline, skipping what tooling already enforces. It requires a minimal, plain solution: no unnecessary machinery, no duplicated shared primitive. Each finding cites the standard it breaks.
- **Adversarial** traces reachable production failures: a concrete trigger, the wrong result it produces, the site and the probe evidence.

Every axis judges the exact candidate SHA its brief names. A proposal about a seam the diff leaves untouched stays out of the findings.

## Findings

A finding is a defect at a site. Each one carries all six parts:

| Part | Content |
| --- | --- |
| severity | `P0`, `P1`, `P2` or `P3` |
| site | file and line range |
| quoted lines | the lines at the site, verbatim |
| replacement | the exact text that replaces them |
| clause | the ticket, spec, intent or standard clause the lines break |
| reproduce | a command that reaches the defect |

A finding missing a part is prose, and prose never bounces. An observation, an out-of-axis remark or a line of evidence stays out of the findings array.

## Implementer report and disputes

The Standards brief carries each judged ticket's Implementer report: `qa`, its `assumptions` and its changed test and check paths. The Standards axis raises:

- a `P1` sited on the ticket's behaviours when `qa.ran` is false, or `qa.evidence` is empty or names nothing the candidate contains;
- a `P1` sited on the check when a changed test or check path weakens, skips, deletes or loosens a check, test or assertion;
- a `P2` when an assumption contradicts the ticket or its specification.

Every axis's brief carries the findings of the ticket's last bounce and the Implementer's disputes of them. Re-judge each disputed finding on its evidence: drop it when the evidence holds, and raise it again with the reason when it does not. A prior finding of this axis whose site is unchanged and that no dispute answers is silent noncompliance: raise a `P1` sited on the finding.

## Verdict

**WRONG_SPEC** comes first, on the Spec axis only: when the ticket contradicts its specification or the approved intent, the defect is in governing scope, not in the candidate. The Spec result is WRONG_SPEC with the finding and the file to repair; the ticket parks for upstream repair and the Implementer is never bounced for it.

Every other verdict is derived from the findings:

- **FINDINGS** when any finding is `P0` or `P1`, or a `P2` whose reproduce command reaches the defect through a path the product itself writes.
- **PASS** otherwise; a `P2` reachable only through hand-edited input and every `P3` go to the ticket's Comments as notes.

A result whose verdict disagrees with its findings, PASS beside a `P0` to `P2` finding or FINDINGS with none, is malformed, and the axis runs again from scratch.

## What refuses landing

- A result for a SHA other than the candidate's.
- A missing axis.
- A malformed result.
- A mutation of HEAD or the worktree during review.

Every axis of a cadence passes the same candidate, or nothing lands. Any change to the candidate, the ticket, the specification or the intent scope voids every earlier pass, and passes from different attempts never combine.

## Evidence

A reviewer's self-certification is never evidence. A claim of acceptance stands on a cited site, a quoted line or a command's output that anyone can rerun.
