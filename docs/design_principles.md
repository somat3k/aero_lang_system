# AERO Lang System — Design Principles

**Document Type:** Design Philosophy & Coding Standards  
**Version:** 1.0

---

## Overview

This document codifies the core design philosophy of the AERO Lang System and the coding principles that every contributor, user, and downstream project should follow. These principles are not arbitrary rules — each one is grounded in the [Whitepaper](./whitepaper.md) and exists to support AERO's four foundational properties: **Adaptivity, Efficiency, Resilience, and Observability**.

---

## Part I — Language Design Principles

### P1: Explicitness Over Magic

Every side-effect, every resource acquisition, and every world interaction must be visible in a function's type signature. There are no hidden I/O calls, no ambient global state, and no implicit resource cleanup. If a function does something, the type system says so.

```aero
// ✅ Good: effects declared explicitly
fn send_notification(msg: Message) -> Result<(), NotifyError> ! [http, log] { … }

// ❌ Bad: side-effects hidden
fn send_notification(msg: Message) -> Result<(), NotifyError> { … }  // where does the HTTP call go?
```

### P2: Make Illegal States Unrepresentable

Types should be designed so that the program cannot construct values that represent invalid or illegal states. Use sum types (enums), newtype wrappers, and non-empty collection types to encode domain invariants.

```aero
// ✅ Good: the type system prevents an order with zero items
struct Order {
    items: NonEmpty<OrderItem>,
    customer: CustomerId,
}

// ❌ Bad: must remember to validate at runtime
struct Order {
    items: Vec<OrderItem>,   // could be empty — silent invariant
    customer: CustomerId,
}
```

### P3: Errors Are Values

Errors must be returned, not thrown. The `Result<T, E>` type is the primary mechanism for error propagation. Panics are reserved for true programmer errors (assertion violations, invariant breaches) — they are never used for recoverable error conditions.

```aero
// ✅ Good
fn parse_age(s: &str) -> Result<u8, ParseError> { … }

// ❌ Bad: panic for a routine input error
fn parse_age(s: &str) -> u8 {
    s.parse().expect("age must be a number")  // panics on bad input
}
```

### P4: Zero-Cost Abstractions

Abstractions — generics, higher-order functions, world bindings, effect handlers — must compile to code equivalent to what an expert would write by hand. Users should never have to choose between clean code and performant code.

### P5: Composition Over Configuration

AERO programs should be built from small, single-purpose components composed together, rather than from monolithic components controlled by large configuration objects. Effect handlers are the primary composition mechanism.

### P6: Stability of Interfaces, Evolution of Implementations

Public APIs (function signatures, world type schemas, capability declarations) change according to semantic versioning and a formal deprecation process. Internal implementations may change freely without notice.

---

## Part II — Coding Principles

### C1: Name Things Clearly

Names should be clear enough that the code reads like a sentence. Prefer full words over abbreviations. Use domain language (the language of the problem, not the language of the machine).

| ❌ Avoid | ✅ Prefer |
|---------|---------|
| `proc_msg(m)` | `process_incoming_message(message)` |
| `usr_id` | `user_id` |
| `tmp` | `temporary_buffer` |
| `flag` | `is_authenticated` |
| `data` | `sensor_reading` |

### C2: One Concept Per Module

Each module should encapsulate exactly one concept, domain entity, or service boundary. A module that does two unrelated things should be split into two modules.

### C3: Keep Functions Small and Focused

A function should do one thing and do it well. If a function requires a comment to explain what it is doing, that is a signal it should be decomposed. A good target: most functions should fit on one screen (~50 lines).

### C4: Declare World Interactions at the Top of the Call Stack

World bindings should be resolved near the entry point of the program, not deep inside domain logic. Domain logic functions should receive already-observed world values as parameters — this keeps them pure, testable, and reusable.

```aero
// ✅ Good: domain logic is pure; world interaction is at the boundary
fn main() ! [temperature_sensor, log] {
    know reading = temperature_sensor.observe();
    know result = domain::process_temperature(reading);
    emit log::info("processed", { result });
}

// ❌ Bad: domain logic reaches out to the world directly
fn process_temperature() ! [temperature_sensor, log] {
    know reading = temperature_sensor.observe();  // world interaction buried in domain
    …
}
```

### C5: Handle Every Error Path

Every `Result` value must be either:
- Propagated upward with `?`,
- Matched and handled explicitly,
- Or explicitly dismissed with `know _ = expr` with a comment explaining why it is safe to ignore.

In AERO, `know _` is the idiomatic discard binding: it uses the same `know` keyword as all other bindings, but the underscore pattern signals that the result is intentionally unused. This makes deliberate discards visible in code review, unlike a bare call with no binding at all.

### C6: Write Tests First (or at Least Alongside)

Every public function should have at least one unit test. Functions that interact with the world should have integration tests using `MockAdapter`. Tests are documentation — they show how the code is intended to be used.

### C7: Log at Boundaries, Not Inside Loops

Log entry and exit to significant operations (function calls, actor messages, world observations). Do not log on every iteration of a tight loop. Use metrics (counters, histograms) for high-frequency numerical data.

---

## Part III — Effect Discipline

### E1: Narrow Effect Sets

Function effect sets should be as narrow as possible. A function that only reads from a database should not declare a `log` effect unless it actually emits logs. Narrow effect sets make functions easier to test and easier to compose.

### E2: Handle Effects at the Right Boundary

Effects should be handled (resolved to concrete implementations) at service boundaries — the outermost point before the request enters your domain logic, or the innermost point before it crosses to an external system. Do not handle effects deep inside domain logic.

### E3: Never Silence Telemetry Effects

The `log`, `metrics`, and `trace` effects must never be handled by a handler that discards events silently. In tests, use the provided `aero_test::capture_telemetry()` handler to capture events for assertion. In production, always wire up a real exporter.

---

## Part IV — World Model Discipline

### W1: Keep World Types Narrow

A world type should model exactly the slice of the world that a component needs. Do not create a single `AppWorld` type that contains everything — create focused types (`UserStore`, `InventoryFeed`, `ConfigFlags`) and compose them at the boundary.

### W2: Declare Consistency Requirements Explicitly

Always declare the consistency model your component requires. If you need strong consistency, say so. If eventual consistency is acceptable, say so. The compiler will warn if your code makes assumptions inconsistent with the declared model.

### W3: Design for Reconciliation

Components must be written with the assumption that their world model may be momentarily stale or unavailable. Design state machines that handle `WorldUnavailable` gracefully (degraded mode, cached values, or explicit error responses) rather than crashing.

---

## Part V — Security Discipline

### S1: Declare Capabilities You Actually Need

Request only the capabilities your program legitimately requires. A web API server that never reads from the filesystem should not request filesystem capabilities. Principle of least privilege applies at the capability level.

### S2: Never Log Sensitive Data Without Redaction

Fields annotated `@sensitive` must never appear in log output without explicit redaction using the `redact()` function. The compiler will catch violations, but code reviewers should also look for patterns that attempt to work around the annotation.

### S3: Validate All External Inputs

Data arriving from world adapters, HTTP requests, or inter-actor messages must be validated before use in domain logic. Use schema validation types (`#[validated]`) to enforce this at the type level.

### S4: Lockfile Must Be Committed

`Aero.lock` must always be committed to version control. Never delete it, never exclude it from VCS. Supply chain integrity depends on pinned dependency hashes.

---

## Part VI — Identity and Autonomy Principles

### I1: AERO Has Its Own Identity

AERO is not Rust, not Go, not Python, not a scripting language. Every syntax choice, every runtime model, every keyword was selected to serve AERO's specific purpose: building knowledge-surface systems that are autonomous, adaptive, and universally applicable. Do not expect AERO to behave like any other language you know. Read the [Language Identity](./language_identity.md) document before writing or reviewing AERO code.

### I2: Programs Assert Knowledge, Not Variable Slots

AERO uses `know` instead of `let`. This is not cosmetic. A `know` binding is an assertion: *the program holds this piece of knowledge*. It signals that the value is meaningful, named, and potentially shareable — not just a temporary variable. Write bindings as assertions of what the program knows at that point.

```aero
// ✅ AERO: the program holds knowledge
know temperature = sensor.observe();
know is_safe = temperature.value < CRITICAL_THRESHOLD;

// ❌ Not AERO style: asking for a variable
let temperature = sensor.observe();
```

### I3: Programs Share Capabilities Proactively

An AERO program is a **mature, autonomous agent**. It does not wait to be asked what it knows — it pushes knowledge to its subscribers as soon as it is available. Design programs to emit world-model deltas, publish actor messages, and propagate information without requiring explicit polling from consumers.

```aero
// ✅ Good: knowledge flows proactively
actor SensorAgent {
    fn loop() ! [world<Temperature>, actor] {
        know reading = temperature_sensor.observe();
        world::emit(Temperature::Delta { value: reading.value });  // push, don't wait to be asked
        sleep(Duration::seconds(1));
        self.loop();
    }
}

// ❌ Bad: sitting idle waiting to be queried
fn get_temperature() -> Temperature ! [world<Temperature>] {
    temperature_sensor.observe()   // only shares knowledge when explicitly called
}
```

### I4: Environments Are Micro-Environments, Not Clones

Never clone the main program's state into a new environment. Instead, create a **micro-environment** — an isolated execution context that grants a scoped capability set and its own world-type bindings, while accessing the original framework modules. This keeps the framework as the single source of truth and eliminates drift between copies.

See [Language Identity §5](./language_identity.md#5-micro-environments) for the full model.

### I5: The Framework Is Domain-Agnostic; World Types Are Domain-Specific

The AERO framework (AVM, scheduler, GC, actor model, effect system, observability pipeline) has no opinion about your domain. The world-type schemas you define are entirely specific to your domain. This separation means the framework improves independently of your domain logic, and your domain logic composes with any other AERO program.

Keep framework concerns (scheduling, telemetry, capability management) inside the framework. Keep domain knowledge (business rules, physical models, financial logic) inside world types and pure functions. Never mix the two.

---


| Code | Principle | Category |
|------|-----------|----------|
| P1 | Explicitness Over Magic | Language Design |
| P2 | Make Illegal States Unrepresentable | Language Design |
| P3 | Errors Are Values | Language Design |
| P4 | Zero-Cost Abstractions | Language Design |
| P5 | Composition Over Configuration | Language Design |
| P6 | Stability of Interfaces | Language Design |
| C1 | Name Things Clearly | Coding |
| C2 | One Concept Per Module | Coding |
| C3 | Keep Functions Small | Coding |
| C4 | World Interactions at the Boundary | Coding |
| C5 | Handle Every Error Path | Coding |
| C6 | Write Tests First | Coding |
| C7 | Log at Boundaries | Coding |
| E1 | Narrow Effect Sets | Effects |
| E2 | Handle Effects at the Right Boundary | Effects |
| E3 | Never Silence Telemetry | Effects |
| W1 | Keep World Types Narrow | World Model |
| W2 | Declare Consistency Requirements | World Model |
| W3 | Design for Reconciliation | World Model |
| S1 | Least Privilege Capabilities | Security |
| S2 | Redact Sensitive Data | Security |
| S3 | Validate External Inputs | Security |
| S4 | Commit the Lockfile | Security |
| I1 | AERO Has Its Own Identity | Identity & Autonomy |
| I2 | Programs Assert Knowledge, Not Variable Slots | Identity & Autonomy |
| I3 | Programs Share Capabilities Proactively | Identity & Autonomy |
| I4 | Environments Are Micro-Environments, Not Clones | Identity & Autonomy |
| I5 | Framework Is Domain-Agnostic; World Types Are Domain-Specific | Identity & Autonomy |

---

*AERO Lang System Design Principles v1.0*
