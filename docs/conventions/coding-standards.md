---
okf_version: "0.2"
type: Convention
title: "Coding standards"
description: "The four-phase ticket build discipline, structural bounds, test rules, and evidence requirements."
status: active
owner: dcon
concepts: [autokuma]
tags:
  - governance
  - coding-standards
generated:
  by: process:spaces-project-deploy
  at: "2026-10-22T18:22:23+00:00"
updated: "2026-10-22T18:22:23+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-22T18:22:23+00:00"
stale_after: "2027-04-20"
---

# Coding standards

Solve the ticket, and nothing else, with the smallest well-written change that does.

## How a ticket is built

**Phase 1, the raw implementation.** Write the absolute bare minimum code required to satisfy the ticket's primary acceptance criteria. Never write pre-emptive validations or fallback states. Never write tests during this phase. Never write an error check for an input that will already fail loudly on its own.

**Phase 2, the debrief.** Explain the exact mechanics of what was built, plainly, before any complexity is added. The builder's report is this debrief.

**Phase 3, scenario planning.** Instead of patching edge cases, propose the specific scenarios, breaking conditions and sub-tickets the core logic suggests, and stop. They are proposals, not work to do now.

**Phase 4, targeted fortification.** Only an approved scenario is built, as its own ticket, with its tests, error checks and fallbacks for that exact condition. Red-green-refactor is the discipline here, not the default for Phase 1.

Treat every ticket as strictly scoped. Do not guess at future requirements, do not "vibe" the architecture, and do not generate defensive layers nobody asked for.

## What fails a candidate

- **It does more than the ticket.** A feature, cleanup, refactor, helper, fixture or abstraction the ticket did not name.
- **It is bigger than it needed to be.** A small ticket is tens of lines. A file that grows by hundreds of lines for a ticket that named one behaviour, with the number stated.
- **It repeats logic that exists.** Shared primitives are called, never restated locally.
- **It validates an input merely to replace an adequate native error, or catches an error solely to rethrow it.** Add custom handling only when the approved scenario requires different behavior.
- **It defends against what the ticket did not name.** Refusing tests for inputs the ticket's Refuses table does not list are Phase 4 work without a Phase 3 approval.
- **It is not plain.** A generator, a queue of continuations, a two-driver abstraction, or a name that needs a paragraph to explain, where a function would do.
- **It cures a refusal by accepting less.** A gate, validator or bridge that skips a check, defaults a required field, or accepts less evidence for one role, where the ticket names no contract clause that exempts that role. The value belongs to a producer; the fix is at the producer.
- **It narrates its own history.** Dates, ticket numbers, decision numbers and the story of why a line exists do not belong in code, comments or standards. History lives in commit messages. A comment says what the code does when the code cannot say it itself.

## Structure

- **Size is bounded.** A source file stays under 1,500 lines of code. A change that crosses the bound splits the unit first. A test file is bounded by its run time and by the same 1,500 lines of code.
- **Dependencies point one way.** Modules sit in declared layers and import only from layers below. An import cycle is a defect.
- **Primitives are leaves.** A shared primitive imports no module that uses it.
- **Each fact has one definition.** A type, constant, vocabulary or parser is defined once and imported everywhere else.
- **Code lives in files.** No program text in strings, no `eval`, no generated script bodies; a subprocess runs a real file.
- **Comments explain intent, not history.** No ticket numbers, decision numbers, run numbers or dates.
- **Every error has a producer.** An error or refusal code exists only if a real caller can raise it and a test shows it.
- **A fix replaces.** A fix removes or rewrites the faulty logic; a fix that only adds lines states why nothing could go.
- **A test exists only for a behaviour a run depends on.** One behaviour, one test; distinct conditions of one rule are rows inside that test. A refusal native failure already produces gets neither a custom check nor a test. A bounce corrects the test that missed; it adds one only when no test covers that behaviour.
- **Tests are cheap and behavioural.** A test file runs in under a minute and asserts behaviour, never source or comment text. Real repositories, processes and networks appear only in tests of that boundary.
- **Durable records have a lifecycle.** Anything written repeatedly carries a stable key and a clearing path, delivered with the writer.
- **Checks enforce structure.** These rules run as deterministic checks. Review judges what the ticket states plus the seams the frozen diff creates or reorders.

A seam the frozen diff creates or reorders is judged on the Spec axis whether or not the ticket names it: a finding is blocking when it names a concrete input a run can reach and the wrong result it produces. A risk is hypothetical only when no reachable input produces it; naming the reachable input and wrong result is evidence, and without them the risk stays nonblocking. A seam the diff does not touch stays a nonblocking proposal. Report the same defect class at an untouched site as a nonblocking sibling note, never as a finding. Standards still requires a documented standard or baseline smell to support a correctness claim; otherwise it records an out-of-axis note.

## TypeScript 7 & `.mts` testing standards

- **TypeScript 7 (`tsgo`) compiler & syntax.** Configure `tsconfig.json` with an explicit `"rootDir"`, `"module": "nodenext"`, `"moduleResolution": "nodenext"`, `"erasableSyntaxOnly": true`, `"verbatimModuleSyntax": true`, `"noImplicitOverride": true`, and `"noFallthroughCasesInSwitch": true`. Rely on TS 7 defaults (`"strict": true`, `"target": "es2025"`, `"alwaysStrict": true`, `"libReplacement": false`). Never use removed TS 6 deprecations (`"baseUrl"`, `"outFile"`, `"downlevelIteration"`, legacy `"moduleResolution": "node"` or `"classic"`, or `import ... assert { type: "json" }` — use `with { type: "json" }`). Ban runtime-emitting TypeScript constructs (`enum`, `namespace`, constructor parameter properties) in favour of `as const` objects with derived union types and explicit class fields. Enforce explicit file extensions (`.mts` or `.mjs`) on relative imports and explicit `import type` for type-only imports. Never use legacy JS TypeScript compiler API runners (`ts-node`, `ts-jest`, or runtime `import ts from "typescript"` hooks).
- **VM-friendly `.mts` test execution & caching.** Prefix test and drain commands with a shared persistent V8 bytecode cache on local RAM disk (`NODE_COMPILE_CACHE=/tmp/node-compile-cache`) — never create or delete per-test ephemeral `mkdtempSync` compile caches. Cap compiler and test worker concurrency on VMs (`--checkers 2` / `--builders 2`, `--test-concurrency=2` to `4`, or `VERIFY_WORKERS=10` with round-robin directory bucket scheduling) so worker spawns do not saturate vCPUs.
- **Black-box entry-point testing & zero internal mocking.** Test packages and modules through their public entry points (`index.mts`, CLI/module seams) as black-box behavioral tests; do not maintain 1:1 internal helper test sprawl or monolithic `export *` barrel chains. Never mock internal modules (`vi.mock` / `mock.module` on internal helpers) or assert internal call counts; inject fakes only at external system boundaries (HTTP, DB, filesystem, clock). Collapse repetitive input variations into table-driven parameterized tests (`for (const tc of cases)` or `test.each`). When using `dependency-cruiser`, disable `tsPreCompilationDeps` and keep boundary checks in top-level `check` rather than the inner test loop.
- **Zero unsafe type assertions.** Ban `any` and `as unknown as T` double-assertions in production and test code. Narrow `unknown` with type guards or schema validators at boundaries, and shrink test setup bloat with `@total-typescript/shoehorn` (`fromPartial(...)`) or strongly-typed factory helpers (`createTestX(overrides?: Partial<X>): X`).

## What does not fail a candidate

- A missing defence the ticket did not ask for. That is a Phase 3 proposal in the report.
- Style below the level of a reader's understanding. Formatting is the formatter's.

## Evidence before a claim

A claim about work state — tests pass, the build succeeds, the bug is fixed, an agent finished, the ticket is met — rests on a command run for that claim and its output read, exit code included. A run from earlier, a partial run, a linter standing in for the compiler, a delegated agent's own report or wording such as "should pass" is not evidence: name the command and what it showed, or state the claim unproven. A regression test counts once it has gone red without the fix and green with it. A delegated change is read in the diff before it is reported.

## Bounces

A bounce is a failure of the system, not a normal step. Every bounce names which language failed — the ticket's, the spec's, the builder brief's, or the grader's understanding — and that cause is read before the next ticket of the same kind is cut. The drain aims at zero bounces.
