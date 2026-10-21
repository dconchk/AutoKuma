---
okf_version: "0.2"
type: Convention
title: "Codebase design"
description: "Vocabulary and principles for deep modules, seams, adapters, red flags, deepening, and design-it-twice."
status: active
owner: dcon
concepts: [autokuma]
tags:
  - governance
  - codebase-design
generated:
  by: process:spaces-project-deploy
  at: "2026-10-21T05:39:20+00:00"
updated: "2026-10-21T05:39:20+00:00"
verified:
  - by: process:spaces-project-deploy
    at: "2026-10-21T05:39:20+00:00"
stale_after: "2027-04-19"
---

# Codebase design

Design **deep modules**: a lot of behaviour behind a small interface, placed at a clean seam, testable through that interface. Use this language and these principles wherever code is being designed or restructured. The aim is leverage for callers, locality for maintainers, and testability for everyone.

## Vocabulary

Use these terms exactly: don't substitute "component," "service," "API," or "boundary." Consistent language is the whole point.

**Module**: anything with an interface and an implementation. Deliberately scale-agnostic: a function, class, package, or tier-spanning slice. _Avoid_: unit, component, service.

**Interface**: everything a caller must know to use the module correctly: the type signature, but also invariants, ordering constraints, error modes, required configuration, and performance characteristics. _Avoid_: API, signature (too narrow, they refer only to the type-level surface).

**Implementation**: what's inside a module, its body of code. Distinct from **Adapter**: a thing can be a small adapter with a large implementation (a Postgres repo) or a large adapter with a small implementation (an in-memory fake). Reach for "adapter" when the seam is the topic; "implementation" otherwise.

**Depth**: leverage at the interface. The amount of behaviour a caller (or test) can exercise per unit of interface they have to learn. A module is **deep** when a large amount of behaviour sits behind a small interface, **shallow** when the interface is nearly as complex as the implementation.

**Seam** _(Michael Feathers)_: a place where you can alter behaviour without editing in that place; the *location* at which a module's interface lives. Where to put the seam is its own design decision, distinct from what goes behind it. _Avoid_: boundary (overloaded with DDD's bounded context).

**Adapter**: a concrete thing that satisfies an interface at a seam. Describes *role* (what slot it fills), not substance (what's inside).

**Leverage**: what callers get from depth. More capability per unit of interface they learn. One implementation pays back across N call sites and M tests.

**Locality**: what maintainers get from depth. Change, bugs, knowledge, and verification concentrate in one place rather than spreading across callers. Fix once, fixed everywhere.

**Information leakage**: one design decision encoded in two or more modules, such as a file format that both the reader and the writer know. Changing the decision means changing every module that knows it. **Back-door leakage** is the worst kind: the shared knowledge appears in no interface, so nothing in the signatures shows it. The opposite of locality.

## Principles

- **Depth is a property of the interface, not the implementation.** A deep module can be internally composed of small, mockable, swappable parts; they just aren't part of the interface. A module can have **internal seams** (private to its implementation, used by its own tests) as well as the **external seam** at its interface.
- **The deletion test.** Imagine deleting the module. If complexity vanishes, it was a pass-through. If complexity reappears across N callers, it was earning its keep.
- **The interface is the test surface.** Callers and tests cross the same seam. If you want to test *past* the interface, the module is probably the wrong shape.
- **One adapter means a hypothetical seam. Two adapters means a real one.** Don't introduce a seam unless something actually varies across it.
- **Repair a leak in one of two ways.** Merge the modules that share the knowledge, or extract the knowledge into a new module. Extract only if the new module gets a small interface. Otherwise the leak just moves, and callers have one more module to learn.

## Designing for testability

1. **Accept dependencies, don't create them.**
2. **Return results, don't produce side effects.**
3. **Small surface area.** Fewer methods = fewer tests needed. Fewer params = simpler test setup.

## Red flags

Named signs of a shallow or leaky module. Use these names when you record friction or write up a deepening candidate:

| Flag | What it looks like |
| --- | --- |
| **Shallow Module** | The interface costs about as much to learn as the implementation saves. Some modules are shallow by design: thin adapters, data classes with no logic, configuration loaders. |
| **Information Leakage** | One design decision is encoded in two or more modules, such as a file format that both the reader and the writer know. |
| **Back-door Leakage** | Modules share knowledge that appears in no interface. Nothing in the signatures shows it. |
| **Temporal Decomposition** | Modules split by execution order (read, then parse, then write), so the same knowledge sits in several of them. |
| **Overexposure** | The common case makes callers learn rarely used options first. |
| **Pass-Through Method** | A method only forwards its arguments to another method with the same signature. |
| **Special-General Mixture** | Code for one use case sits inside a general mechanism that every caller shares. |
| **Conjoined Methods** | Two methods make sense only when read together. They are one module under two names. |
| **Hard to Describe** | A complete, simple description of the module is hard to write. The difficulty is the finding. |
| **Hard to Pick Name** | No name fits, usually because the module holds two concepts. Split it, and each part names itself. |
| **Classitis** | Many small, shallow modules add up to a large total interface. Splitting a shallow module further makes this worse. |

One flag on one module is noise. A candidate is several flags on the same group of modules.

## Deepening

When assessing a candidate for deepening, classify its dependencies to decide how the deepened module is tested across its seam:

1. **In-process**: Pure computation, in-memory state, no I/O. Always deepenable: merge the modules and test through the new interface directly. No adapter needed.
2. **Local-substitutable**: Dependencies with local test stand-ins (PGLite for Postgres, in-memory filesystem). Tested with the stand-in running in the test suite; the seam is internal.
3. **Remote but owned (Ports & Adapters)**: Your own services across a network boundary. Define a port (interface) at the seam; the deep module owns the logic and the transport is injected as an adapter (in-memory for tests, HTTP/gRPC/queue for production).
4. **True external (Mock)**: Third-party services you don't control. Inject as a port; tests provide a mock adapter.

**Testing strategy (replace, don't layer)**: Old unit tests on shallow modules become waste once tests at the deepened module's interface exist; delete them in their own commit after the new interface tests are green.

## Design it twice

When exploring alternative interfaces for a chosen deepening candidate:

1. **Frame the problem space**: state the constraints, dependency category, and a rough illustrative sketch for the operator.
2. **Explore 3+ radically different interfaces** (via parallel sub-agents when available):
   - Brief 1: Minimize the interface (1–3 entry points max, maximum leverage per entry point).
   - Brief 2: Maximize flexibility (support extension and diverse callers).
   - Brief 3: Optimize for the most common caller (make the default case trivial).
   - Brief 4 (if applicable): Design around ports & adapters for cross-seam dependencies.
3. **Compare and recommend**: contrast by **depth**, **locality**, and **seam placement**, and recommend the strongest design (or a hybrid).
