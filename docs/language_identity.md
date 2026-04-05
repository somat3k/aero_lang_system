# AERO Language Identity

**Document Type:** Language Philosophy & Syntax Specification  
**Version:** 1.0

> *"A mature program does not ask for permission. It knows what it is, shares what it has, and serves the world it was built for."*

---

## Overview

This document defines what AERO is and, equally importantly, what it is not. Every language design decision — from syntax keywords to the runtime execution model — flows from a single thesis:

**AERO is a knowledge surface.** It is a framework for summoning, holding, and sharing structured knowledge about the real world — across domains, across environments, across users — in a way that is autonomous, adaptive, and universally applicable.

AERO is not Rust. It is not Go. It is not a scripting language with types bolted on, nor a functional language with effects bolted on. It is its own thing, designed for a purpose none of those languages were designed for.

---

## Table of Contents

1. [What AERO Is Not](#1-what-aero-is-not)
2. [The Knowledge Surface Model](#2-the-knowledge-surface-model)
3. [Program Autonomy](#3-program-autonomy)
4. [AERO Syntax: Distinct by Design](#4-aero-syntax-distinct-by-design)
5. [Micro-Environments](#5-micro-environments)
6. [Universal Domain Adaptability](#6-universal-domain-adaptability)
7. [Parallel Sharing with Isolation](#7-parallel-sharing-with-isolation)
8. [Language Identity Quick Reference](#8-language-identity-quick-reference)

---

## 1. What AERO Is Not

Understanding what AERO is requires first being clear about what it is not — because every existing language carries design assumptions that do not belong in AERO.

### Not Rust

Rust's primary goals are memory safety without garbage collection and zero-cost systems programming. Its syntax — borrow checker, `let` bindings, lifetimes, traits — serves those goals. AERO has different goals: knowledge representation, world-model adaptivity, and autonomous program behaviour. Rust's syntax would be the wrong fit and the wrong signal.

Specific differences:

| Concern | Rust Approach | AERO Approach |
|---------|--------------|--------------|
| Value binding | `let x = value` — asks permission | `know x = value` — asserts knowledge |
| Mutability | `let mut x` — exception to immutability | `know mut x` — capability of the binding |
| Memory | Manual borrow checker | Linear types + AVM GC |
| Concurrency | Threads + `Arc<Mutex<T>>` | Actors + typed mailboxes |
| Error handling | `Result<T,E>` + `?` (borrowed from AERO's predecessors) | `Result<T,E>` + `?` + typed fault hierarchy |
| World interaction | No concept | First-class `world<W>` types |

### Not Go

Go's goals are simplicity, fast compilation, and easy concurrency. Its goroutine model, lack of generics (early versions), and explicit error returns are suited to building servers quickly. AERO builds on the lessons of Go's concurrency model but adds a rich type system, a world-model layer, and an entirely different execution philosophy.

### Not a Functional Language

AERO has immutable defaults and a strong effect system, but it is not purely functional. Programs are not sequences of mathematical transformations. They are **autonomous agents** that observe the world, reason about it, act on it, and share their knowledge with other agents — continuously, in real time, across any domain.

### Not a Scripting Language

AERO compiles to typed bytecode (AVM) or native code (LLVM backend). It has static types, a formal type theory, and a structured execution model. It is designed for production systems at enterprise scale, not for one-off automation scripts.

---

## 2. The Knowledge Surface Model

The central metaphor of AERO is the **knowledge surface** — a living, structured representation of everything the program knows about its world, shared across all agents that interact with it.

Imagine the universe as an information space. Every real-world entity — a temperature sensor, a financial market, a user preference, a network topology — has a representation in this information space. The job of an AERO program is not to "query a database" or "call an API" — it is to **hold knowledge** about its slice of the world and **share that knowledge** with whoever needs it.

```
Universe Information Space
─────────────────────────────────────────────────────────────────
│  Physical world    │  Digital world    │  User world          │
│  (sensors, IoT)    │  (services, APIs) │  (preferences, state)│
─────────────────────────────────────────────────────────────────
                             │
                     AERO Knowledge Surface
                             │
              ┌──────────────┼──────────────────┐
              │              │                  │
         World Types    Active Agents      Micro-Environments
     (structured          (share &          (user-paced,
      knowledge)          invoke)             isolated)
```

This is what the `world` type system represents: **typed, versioned, observable knowledge** about a slice of the universe. When a program reads from a `world<Temperature>`, it is not making a network call — it is accessing its current knowledge of the temperature domain, with the runtime ensuring that knowledge is kept as fresh as the declared consistency model requires.

### Knowledge Is Active, Not Passive

Knowledge in AERO is **active** — it is always being reconciled, always pushing updates to subscribers, always available for invocation. The program does not poll for knowledge; knowledge arrives at the program. This is the distinction between AERO's world model and a traditional database query.

---

## 3. Program Autonomy

An AERO program is a **mature, confident agent**. It does not ask permission at every step. It knows what it holds, shares what it has, and acts within its declared capability set without requiring supervision for every individual action.

This philosophy is expressed at every level of the language:

### Capabilities Are Declared Once

An AERO program declares its capabilities in `Aero.toml` — once, at the package level. Inside the program, those capabilities are available as first-class objects, automatically wired by the runtime. The program does not ask "may I access the network?" on every request — it simply uses its `NetworkCap`, which the runtime granted at startup.

```aero
// Capabilities declared once in Aero.toml:
// [capabilities]
// network = ["outbound:https"]
// world = ["temperature_sensor"]

// Inside the program — no permission-asking, just use:
fn main() ! [temperature_sensor, http, log] {
    know reading = temperature_sensor.observe();
    know response = http.post("https://alerts.example.com", reading.to_json()) ? ;
    emit log::info("alert sent", { status: response.status });
}
```

### Programs Share Proactively

Rather than waiting to be queried, AERO programs **push knowledge** to subscribers. An actor that holds sensor data does not sit idle waiting for requests — it emits world-model deltas as they occur. Consumers subscribe and receive updates at their own pace.

```aero
actor TemperatureKnowledgeHolder {
    fn loop(world: TemperatureSensor) ! [world<TemperatureSensor>, actor] {
        know reading = world.observe();
        // proactively share with all registered subscribers
        world.emit(TemperatureSensor::Delta { value: reading.value, at: Instant::now() });
        sleep(Duration::seconds(1));
        self.loop(world);
    }
}
```

### The `know` Keyword

The `let` keyword — borrowed from mathematical notation "let x be..." — implies a tentative assignment, a request to the compiler to introduce a binding. It feels uncertain. It asks permission.

AERO uses `know` instead. When an AERO program binds a value, it **knows** that value. The binding is an assertion of knowledge, not a request for a variable slot.

```aero
// AERO: the program asserts knowledge
know temperature = sensor.observe();
know normalised  = convert_to_celsius(temperature);
know is_critical = normalised.value > THRESHOLD;
```

This is not cosmetic. It reflects a fundamental difference in program philosophy:
- `let` → "I am setting up a temporary variable"
- `know` → "I am holding a piece of structured knowledge, and I am prepared to share it"

---

## 4. AERO Syntax: Distinct by Design

AERO's syntax is built from first principles to serve its philosophy. Every keyword and construct is chosen to express AERO concepts — not borrowed from other languages for familiarity.

### Binding: `know`

| Pattern | Syntax | Notes |
|---------|--------|-------|
| Immutable binding | `know x = value` | Default — most bindings are immutable |
| Mutable binding | `know mut x = value` | Mutation is an explicit, stated capability |
| Typed binding | `know x: Type = value` | Type annotation when inference is insufficient |
| Destructuring | `know (a, b) = pair` | Tuple destructuring |
| Pattern binding | `know Some(inner) = option else { return; }` | Guarded pattern bind |
| Discard | `know _ = expr` | Explicitly discard a value — makes intentional discards visible in review |

```aero
know sensor_id: String = "sensor-01".to_string();
know mut count = 0;
know (latitude, longitude) = gps.position();
know Some(user) = users.find(id) else {
    return Result::Err(NotFound(id));
};
```

### Functions: `fn`

Functions use `fn` — the mathematical notation for mapping — with effect declarations:

```aero
fn convert_temperature(reading: Temperature, target: TemperatureUnit) -> Temperature ! [] {
    // pure function — no effects, no world access
    …
}
```

### World Interaction

World interactions use the effect system with clear keywords:

| Action | Syntax |
|--------|--------|
| Observe world state | `world_name.observe()` |
| Emit a world delta | `world_name.emit(delta)` |
| Subscribe to changes | `world_name.subscribe()` |

### Pattern Matching

Pattern matching uses `match` (familiar from ML-family languages, distinct from C/Java `switch`):

```aero
match unit {
    TemperatureUnit::Celsius    => reading.value,
    TemperatureUnit::Fahrenheit => (reading.value - 32.0) * 5.0 / 9.0,
    TemperatureUnit::Kelvin     => reading.value - 273.15,
}
```

### Error Propagation

Errors propagate with `?`, which short-circuits on `Err`, consistent with the typed fault model:

```aero
know result = fallible_operation() ? ;  // propagates Err to caller
```

### Complete Syntax Example

A complete AERO function, showing how all elements compose:

```aero
/// Processes incoming sensor readings and shares alerts with the world.
fn process_and_share(
    reading: Temperature,
    threshold: Float64,
) -> Result<(), AlertError> ! [alerts, log] {
    know celsius = convert_temperature(reading, TemperatureUnit::Celsius);

    emit log::debug("reading processed", {
        sensor: celsius.sensor_id,
        value:  celsius.value,
    });

    if celsius.value > threshold {
        know alert = Alert {
            sensor_id: celsius.sensor_id,
            value:     celsius.value,
            threshold,
            fired_at:  Instant::now(),
        };
        alerts.emit(Alert::Delta::NewAlert(alert)) ? ;
    }

    Result::Ok(())
}
```

---

## 5. Micro-Environments

A key architectural principle of AERO is the **micro-environment model**. Environments in AERO are not copies or clones of one another. They are **isolated execution contexts that each access the original framework's modules and capabilities directly**.

### What a Micro-Environment Is

A micro-environment is a scoped, isolated execution context with:
- Its own actor supervision tree,
- Its own world-type bindings (pointing to real or mock adapters),
- Its own capability grants (a subset of the parent's capabilities),
- Its own telemetry namespace.

What it is **not**:
- A copy of the main program,
- A VM-within-a-VM (full virtual machine),
- A container (no separate kernel context).

### Micro-Environments vs. Clones

```
Traditional approach (clone-based):
  Main program ──clone──► User A environment  (full copy)
               ──clone──► User B environment  (full copy)
               ──clone──► User C environment  (full copy)

  Problems: 3x memory, drift between copies, update all three separately.

AERO micro-environment approach:
  Main program (original modules, original capabilities)
       │
       ├── User A micro-env (own world bindings + capabilities)
       ├── User B micro-env (own world bindings + capabilities)
       └── User C micro-env (own world bindings + capabilities)

  Each micro-env accesses the same original functions.
  No duplication. Updates propagate to all environments immediately.
```

### Creating a Micro-Environment

```aero
// Launch a micro-environment for a specific user context
fn spawn_user_environment(user_id: UserId, user_prefs: UserPreferences) -> EnvRef ! [actor] {
    know env = MicroEnvironment::new(user_id)
        .with_world(UserState::adapter_for(user_id))
        .with_capability(CapabilitySet::for_user(user_prefs))
        .with_telemetry_namespace(format!("user.{}", user_id));

    spawn_env(env, user_session_loop)
}
```

### Adaptability and Compatibility

Each micro-environment **measures its adaptability** through the capability compatibility system. When a user requests a function, the runtime checks:

1. Does the user's micro-environment hold the required capability?
2. Is the requested functionality compatible with the user's declared world bindings?
3. Are the effect requirements of the called function satisfied by the environment's grants?

If all checks pass, the function executes in the user's own paced environment. If not, the system returns a typed `CapabilityMismatch` error — not a silent failure.

---

## 6. Universal Domain Adaptability

AERO is explicitly designed to be **domain-agnostic at the language level** and **domain-specific at the world-type level**. The same language, the same runtime, the same capability model — different world-type schemas.

### Domain Adaptors in Practice

| Domain | World Type Schema | Micro-Environment Role |
|--------|-----------------|----------------------|
| **Science** | `world PhysicsSimulation { … }` | Each simulation run is a micro-env with its own parameters |
| **Finance** | `world MarketFeed { price: Decimal, … }` | Each trader's workspace is a micro-env with their own risk limits |
| **AI/ML** | `world ModelKnowledge { weights: Tensor, … }` | Each model experiment is a micro-env with its own training state |
| **IoT** | `world SensorNetwork { devices: Vec<Device>, … }` | Each deployment site is a micro-env with its own sensor bindings |
| **Healthcare** | `world PatientRecord { … }` | Each patient session is a micro-env with HIPAA-compliant capability grants |

The AERO framework does not know or care whether you are simulating physics, trading equities, or training neural networks. Its job is to:
1. Give you a type-safe world-model for your domain,
2. Reconcile your program's knowledge of that domain with reality,
3. Share updates across all agents that hold knowledge of the same world,
4. Do all of this with zero-overhead abstractions and intrinsic observability.

### Domain Example: AI From a Calculator

A user wanting to build an AI system starts with the same AERO framework as a user building a calculator. The calculator micro-environment holds a `world<MathContext>`. The AI user's micro-environment holds a `world<ModelKnowledge>` and an additional `world<TrainingData>`. The original framework modules — actor scheduling, world reconciliation, telemetry, capability management — are shared. Only the world-type schemas differ.

```aero
// Calculator micro-environment
world MathContext {
    expression: String,
    result:     Option<Decimal>,
    history:    Vec<Calculation>,
}

// AI research micro-environment — different world, same framework
world ModelKnowledge {
    architecture: ModelArchitecture,
    weights:      Tensor,
    training_loss: Float64,
    validation_acc: Float64,
    epoch: u32,
}
```

Both run on the same AVM, the same scheduler, the same GC, the same observability pipeline.

---

## 7. Parallel Sharing with Isolation

AERO supports **parallel knowledge sharing** — multiple agents holding and sharing knowledge simultaneously — while maintaining **isolation** between each agent's execution context.

### The Blocking Model

In AERO, "blocking" at the environment level is a deliberate design tool. When a micro-environment reaches the boundary of its declared capabilities, it **blocks gracefully**: it does not crash, does not escalate silently, and does not leak state. It returns to its declared degraded-mode behaviour (see [Design Principles W3](./design_principles.md#W3)).

This blocking mechanism enables:
- **Safe learning**: a new user's micro-environment can experiment with framework capabilities without affecting other users,
- **Parallel passing**: knowledge can be passed (via typed actor messages) between micro-environments in parallel, with each environment consuming at its own pace (back-pressure applied automatically),
- **Declaration escape**: the blocking boundary is also the point where the strict declarative contract ends and imperative execution takes over — the micro-environment's own actor can respond to real-time events in any order.

### Concurrent Knowledge Propagation

```
World<MarketFeed> (one source of truth)
        │
        │  emit(delta)
        ▼
World-Model Reconciliation Engine
        │
        │  broadcasts to all subscribers
        ├──────────────────────────────┐
        ▼                             ▼
  Trader A micro-env            Trader B micro-env
  (processes at own pace)       (processes at own pace)
  (own risk limits)             (own risk limits)
  (isolated execution)          (isolated execution)
```

Neither Trader A nor Trader B blocks the other. Neither can access the other's state. Both receive the same market knowledge, at their own pace, through their own capability-gated environment.

---

## 8. Language Identity Quick Reference

| Concept | AERO Choice | Rationale |
|---------|------------|-----------|
| Value binding keyword | `know` | Programs assert knowledge, not request variable slots |
| Mutable binding | `know mut` | Mutability is a declared capability, not a default escape hatch |
| Function definition | `fn` | Mathematical mapping notation |
| World interaction | `world_name.observe()` / `.emit()` | Explicit, typed, effect-tracked |
| Concurrency model | Actors + typed mailboxes | Isolation without shared-state complexity |
| Error model | `Result<T,E>` + `?` | Errors are values, never exceptions |
| Resource management | Linear types + AVM GC | Safety without manual lifecycle management |
| Isolation model | Micro-environments | Isolated contexts, shared original framework |
| Knowledge model | World types + reconciliation | Typed, versioned, always-fresh world knowledge |
| Capability model | Declared in `Aero.toml`, granted at startup | Least privilege, no ambient authority |

---

*AERO Language Identity Specification v1.0*
