# AERO Lang System — Glossary

**Document Type:** Terminology Reference  
**Version:** 1.0

---

Terms are listed alphabetically. Cross-references to other glossary entries are written in *italics*.

---

## A

**Actor**  
The fundamental unit of concurrency in AERO. An actor is an isolated, independently-scheduled process that communicates exclusively by sending and receiving typed messages. Actors have their own stack and mailbox; they share no mutable state with other actors. See: *Actor Model*, *Supervision Tree*.

**Actor Model**  
A mathematical model of concurrent computation (Hewitt, 1973) in which computation is expressed as actors that send messages, create new actors, and decide how to behave in response to received messages. AERO's concurrency model is based on the actor model extended with typed mailboxes and supervision.

**Actor Mesh**  
The live graph of all actors running in an AVM process (or cluster), their links, and their monitor relationships.

**ActorRef\<T\>**  
A lightweight, send-able reference to an actor that accepts messages of type `T`. Sending to an `ActorRef` is non-blocking. `ActorRef`s may be local or remote (in a cluster deployment).

**Adaptivity**  
One of the four AERO core properties. A system is *adaptive* if it can update its behaviour in response to changes in its operating environment without redeployment. In AERO, adaptivity is enabled by the *World-Model Reconciliation Engine* and *Hot-Code Replacement*.

**AVM (AERO Virtual Machine)**  
The register-based virtual machine that executes AERO bytecode. The AVM provides the *Scheduler*, *Garbage Collector*, *Reconciliation Engine*, and *Hot-Code Replacement* mechanism. AERO programs can also compile to native code via the LLVM backend, bypassing the AVM for production performance targets.

**AVM Bytecode**  
The portable, platform-independent binary representation of a compiled AERO program. Bytecode files use the `.avm` extension and are loaded by the *AVM* at runtime.

**aeroc**  
The AERO compiler CLI. Provides `check`, `build`, `run`, `fmt`, `test`, and `new` subcommands.

**Aero.toml**  
The *package manifest* file. Declares package metadata, *capabilities*, dependencies, and build profiles.

**Aero.lock**  
The *lockfile*. Records the exact version and content hash of every transitive dependency. Must be committed to version control. Never manually edited.

---

## B

**Back-Pressure**  
A flow-control mechanism by which a consumer signals a producer to slow down when the consumer's buffer is full. Back-pressure prevents unbounded queue growth and memory exhaustion in streaming pipelines. AERO's actor mailboxes support configurable back-pressure policies.

**Bump-Pointer Allocator**  
A fast memory allocator used for the *Nursery* (Gen0) generation in the *AVM GC*. Allocation is a single pointer increment; deallocation is performed by the next GC cycle.

---

## C

**Capability**  
A runtime permission token that grants a program access to a specific resource or operation (e.g., outbound HTTP, filesystem read). Capabilities are declared in `Aero.toml` and enforced at both compile time (effect types) and runtime (the AVM capability verifier). See *Capability-Based Security*.

**Capability-Based Security**  
A security model in which access to resources is governed by possession of a capability token, rather than identity-based access control. AERO implements capability-based security for all external resource access.

**Circuit Breaker**  
A resilience pattern that monitors calls to an external resource and "trips" (stops calling) when the error rate exceeds a threshold. AERO provides `circuit<T>(expr, fallback)` as a built-in language construct. See *Resilience Primitives*.

**Cluster Protocol**  
The binary-over-TLS protocol used for communication between AERO nodes in a distributed deployment. Enables remote *ActorRef* addressing and inter-node message passing.

**Compiler Attribute**  
A declarative annotation applied to a declaration (function, struct, enum, field) to instruct the compiler to generate additional code or enforce additional rules. Examples: `#[derive(Debug)]`, `#[sensitive(PII)]`.

**Consistency Model**  
A specification of the guarantees that a *World Type*'s *World Adapter* makes about the freshness and ordering of observations. AERO supports three built-in consistency models: `strong`, `causal`, and `eventual`.

---

## D

**Data Classification**  
The practice of annotating data values with their sensitivity category (e.g., `@sensitive(PII)`) so that the compiler and runtime can enforce access control, redaction, and audit requirements.

**DDAC (Discovery-Dynamics-Adaptable Computation)**  
HoloLang's core computation model for physical device systems. DDAC operates in three phases: ① **Discover** — probe sensors and characterise the environment to build an initial computational profile; ② **Dynamics** — continuously observe environmental drift and track computation graph performance; ③ **Adapt** — recompile the hot path in a shadow micro-environment and hot-swap the execution plan when the new plan is validated. DDAC ensures that computation graphs transition smoothly when conditions change — no restarts, no abrupt mode switches. See: *HoloLang*, *Micro-Environment*.

**Dependent Type**  
A type whose definition depends on a value. AERO's type system extends Hindley-Milner with dependent types to allow types to express invariants about values (e.g., a non-empty list, an integer in range 0–100).

**DocDirectory**  
A structured documentation artefact generated automatically from a HoloLang session's declarations. Contains Markdown and JSON descriptions of every device, computation graph, channel, skill, and enumeration in the session. Declared with the `doc_directory { … }` block. See: *HoloLang*, *Session (HoloLang)*.

**DriftSeverity**  
An enum emitted by the *Reconciliation Engine* classifying how significantly the observed world state has diverged from the last-known state: `Minor`, `Major`, or `Critical`.

---

## E

**Edition**  
A versioned snapshot of the AERO language specification that a package targets. Editions allow the language to evolve without breaking existing packages. The current edition is `2026`.

**Effect**  
A classification of side-effects that a function may produce. Effects are declared in function signatures using the `!` notation and checked by the compiler. Examples: `log`, `http`, `filesystem`, `world<W>`.

**Effect Boundary**  
A point in the program where effects are handled (resolved to concrete implementations). Effect boundaries are the primary mechanism for dependency injection and testability in AERO programs.

**Effect Handler**  
An object that provides a concrete implementation for one or more effects. Passed to `handle … with …` expressions to resolve effects at a boundary.

**Effect Set**  
The set of effects declared in a function's type signature. A function with effect set `[http, log]` may make HTTP requests and emit log events.

**Effect System**  
The part of the AERO type checker that tracks, infers, and verifies effect sets. The effect system ensures that every side-effect is visible in the type of the function that produces it.

**Efficiency**  
One of the four AERO core properties. A system is *efficient* if its abstractions compile to code equivalent to what an expert would write by hand. AERO achieves efficiency through zero-cost abstractions, linear types, and the LLVM native backend.

---

## F

**Feature Flag**  
A runtime boolean value that controls whether a particular feature or code path is enabled. In AERO, feature flags are modelled as *World Types* so that changes propagate to running actors in real time.

---

## G

**GEMM (General Matrix Multiplication)**  
The mathematical kernel underlying HoloLang's computation graph acceleration. All `matmul`, `conv2d`, and linear-algebra-heavy operations in a HoloLang graph are automatically lowered to GEMM kernel descriptors and dispatched to the best available accelerator (GPU via cuBLAS/ROCm, CPU via AVX-512 OpenBLAS, or ARM NEON). A calibration pass at session startup benchmarks available accelerators and builds a DDAC-guided dispatch table. See: *HoloLang*, *DDAC*, *SafeTensor*.

**Garbage Collector (GC)**  
The AVM subsystem responsible for automatically reclaiming memory no longer reachable by the program. AERO uses a tri-colour, incremental, generational GC. *Linear types* allow deterministic deallocation of resources outside GC involvement.

**Green Thread**  
A lightweight user-space thread managed by the *Scheduler*, not the OS. AERO programs can have millions of concurrent green threads (actors) because their stacks are small and managed by the runtime.

---

## H

**HoloLang**  
A domain-specific language that bridges to AERO for holographic projection and physical device orchestration systems. HoloLang source files (`.hl`) are compiled to AERO bytecode by `aeroc --domain holographic`. HoloLang adds: device declarations (type-safe hardware descriptors), *SafeTensor* (compile-time shape-checked tensors), computation graphs (GEMM-accelerated pipelines), *MDI Canvas* (spatial mesh projection model), *DDAC* (Discovery-Dynamics-Adaptable Computation), multi-channel communication (gRPC, WebSocket, webhook, REST), session lifecycle, skill tracking, and *DocDirectory* generation. All HoloLang constructs expand to well-typed AERO actors, world types, and effect-tracked functions — there is no additional runtime layer. See: [HoloLang Specification](./hololang.md).

**Hindley-Milner**  
A classical type inference algorithm that infers the types of expressions without requiring explicit annotations. AERO uses an extended Hindley-Milner algorithm as the foundation of its type inference engine.

**Hot-Code Replacement**  
The ability to load new bytecode into a running AVM process and migrate live actors to the new module version, without restarting the process. Enables zero-downtime deployments and live configuration changes.

---

## I

**Incremental Compilation**  
A compiler optimisation that re-compiles only declarations whose inputs have changed since the last build. AERO's compiler maintains a dependency graph at the declaration level for this purpose.

---

## K

**Knowledge Surface**  
The central architectural metaphor of AERO. A knowledge surface is the live, structured, typed representation of everything a program knows about its world — held as first-class `world` type bindings, continuously reconciled with reality, and shared proactively with all subscribing agents. The knowledge surface unifies what traditionally requires a database, an event bus, and a cache into a single typed, observable abstraction. See: *World Type*, *World-Model Reconciliation Engine*.

---

## L

**`know` (binding keyword)**  
AERO's value-binding keyword. Replaces `let` from Rust-family languages. A `know` binding is an assertion of program knowledge — "the program holds this value and is prepared to share it" — rather than a request for a variable slot. Immutable by default; use `know mut` for mutable bindings. Use `know _ = expr` to explicitly discard a value (making the intentional discard visible in code review). Examples: `know temperature = sensor.observe();`, `know mut count = 0;`, `know _ = side_effect_call(); // result intentionally unused`

**Linear Type**  
A type that must be used exactly once — neither dropped nor duplicated without explicit acknowledgement. AERO uses linear types for resource safety: file handles, network connections, and external service tokens are linear by default.

**Lockfile**  
See *Aero.lock*.

---

## M

**Mailbox**  
The bounded message queue belonging to an *Actor*. Incoming messages are enqueued in the mailbox and processed one at a time by the actor on its scheduled turn.

**MDI Canvas (Multi-Domain Interface Canvas)**  
HoloLang's spatial model for projection systems. An MDI canvas divides the projection space into a mesh of independently addressable tiles, each capable of carrying its own *impulse cycle*. Declared with `canvas { mesh: [R, C], tile_size: [W, H], frame_rate: N }`. Tiles are addressed as `[row, col]` and support individual, row, column, or rectangular-region operations. See: *HoloLang*, *Impulse Cycle*.

**Micro-Environment**  
An isolated execution context within an AERO cluster or process. A micro-environment has its own actor supervision tree, world-type bindings, capability grants (a subset of the parent's), and telemetry namespace. Critically, a micro-environment does **not** copy or clone the parent program's state — it accesses the original framework modules directly, eliminating drift between environments. Multiple micro-environments can run concurrently, each processing knowledge at its own pace. See: *Parallel Sharing*, *Capability*.

**MIR (Mid Intermediate Representation)**  
The compiler's monomorphised, closure-converted, CPS-transformed intermediate representation. MIR is the input to the *Optimiser* and *Code Generator*.

**Module**  
A namespaced compilation unit within a *Package*. Modules are defined by `.aero` files and can be nested.

---

## N

**Newtype**  
A struct wrapping a single value of another type, used to create a distinct type identity without runtime cost. Example: `struct UserId(u64)`.

**Non-Empty Collection (`NonEmpty<T>`)**  
A collection type guaranteed to contain at least one element. Eliminates the need for "is this list empty?" runtime checks in domain code.

**Nursery (Gen0)**  
The youngest generation in the AVM *Garbage Collector*. New allocations go into the nursery; most short-lived objects die here and are collected cheaply.

---

## O

**Observability**  
One of the four AERO core properties. A system is *observable* if its internal state can be fully understood from its external outputs (logs, metrics, traces). AERO makes observability intrinsic: logs, metrics, and traces are language-level constructs.

**OpenTelemetry**  
An open standard for distributed tracing, metrics, and logs. AERO's *Telemetry API* is compatible with OpenTelemetry and exports via the OTLP protocol.

---

## P

**Package**  
A versioned, deployable unit of AERO code. Described by an `Aero.toml` manifest. Packages may be libraries, binaries, or both.

**Package Manifest**  
See *Aero.toml*.

**Pattern Matching**  
A language feature that tests a value against a series of patterns and executes code for the first matching pattern. AERO's `match` expression provides exhaustiveness checking — the compiler ensures every case is handled.

**Program Autonomy**  
An AERO design principle. An AERO program is a mature, confident agent that shares its knowledge and capabilities proactively — without waiting to be asked and without requiring explicit permission at each step. Capabilities are declared once (in `Aero.toml`) and used freely within the program. Knowledge is pushed to subscribers via `world.emit()` rather than pulled by callers. See: *Knowledge Surface*, *Micro-Environment*, *`know` (binding keyword)*.

---

## R

**Reconciliation Engine**  
The AVM background subsystem that continuously compares a program's declared *World Model* against observed reality. When drift is detected, it dispatches `WorldChanged` messages to subscribed actors.

**Resilience**  
One of the four AERO core properties. A system is *resilient* if it degrades gracefully in the presence of partial failures. AERO achieves resilience through *Supervision Trees*, *Circuit Breakers*, *Retry*, *Timeout*, and the *World-Model Reconciliation Engine*.

**Retry**  
A resilience primitive that re-executes a fallible expression up to a specified number of times with exponential back-off. Built into the AERO language as `retry<N>(expr)`.

---

## S

**SafeTensor\<T, Shape\>**  
HoloLang's primary tensor type. The `Shape` parameter is part of the type — shape mismatches between operations (e.g., adding tensors of incompatible sizes, multiplying matrices with incompatible dimensions) are **compile-time errors**, not runtime panics. Supports GEMM-lowered operations (`matmul`, `conv2d`) and vectorised operations (`relu`, `normalize`, element-wise arithmetic). Zero-copy views for transpose and reshape. See: *HoloLang*, *GEMM*.

**Scheduler**  
The AVM subsystem responsible for multiplexing green threads (actors) onto OS threads. AERO uses a work-stealing, M:N cooperative scheduler.

**Semantic Versioning (SemVer)**  
A versioning scheme where version numbers encode the nature of changes: `MAJOR.MINOR.PATCH`. AERO packages follow SemVer; the AERO language and standard library commit to SemVer stability guarantees from v1.0 onward.

**Session (HoloLang)**  
The top-level execution scope in a HoloLang program. A session owns all devices, canvases, channels, and skills declared within it and compiles to an AERO actor supervision tree. Sessions support graceful teardown (devices driven to safe states before termination) and DDAC-guided adaptation throughout their lifetime. Declared with `@session("name")`. See: *HoloLang*, *DDAC*, *Skill*.

**Skill (HoloLang)**  
A named, versioned capability declared in a HoloLang session's `skills { … }` block. Skills are resolved against the AERO framework before session startup — incompatible skills produce structured `SkillResolutionError` values, not runtime crashes. Skill compatibility is checked against version (SemVer), effect requirements, and device type requirements. See: *HoloLang*, *Capability*.

**Supervision Tree**  
A hierarchy of *Actors* where parent actors (supervisors) govern the restart policies of their children. When a child actor crashes, the supervisor decides whether to restart it, stop it, or escalate the failure. Declared using the `supervisor` keyword.

---

## T

**Telemetry**  
Collectively: structured logs, metrics, and distributed traces emitted by a program to make its behaviour observable. In AERO, telemetry is a first-class language concern emitted via the `emit` keyword.

**Timeout**  
A resilience primitive that imposes a deadline on a computation. Built into AERO as `timeout<D>(expr)`.

**Trace Context**  
Metadata propagated across actor boundaries and process boundaries to correlate related operations in a distributed trace. AERO propagates trace context automatically following the W3C TraceContext standard.

---

## W

**Work-Stealing Scheduler**  
A scheduling algorithm where idle OS threads "steal" work from the run-queues of busier threads, balancing load automatically across available CPU cores.

**World Adapter**  
An implementation of the `WorldAdapter<W>` interface that connects a *World Type* to its real-world data source (an HTTP endpoint, a Kafka topic, a database table, etc.).

**World Binding**  
A named association between a *World Type* and a *World Adapter*, declared in `Aero.toml`. Programs reference world bindings as effects in their function signatures.

**World Model**  
A program's typed, runtime representation of the real-world entities it interacts with. Modelled as *World Types* and kept current by the *Reconciliation Engine*.

**World Type**  
A struct-like type declaration prefixed with `world`. World types declare the schema, consistency model, and change protocol of a real-world entity that the program observes or modifies.

**WorldChanged\<W\>**  
A message dispatched by the *Reconciliation Engine* to actors that have subscribed to changes in world `W`. Carries the previous state, current state, computed delta, and drift severity.

---

## Z

**Zero-Cost Abstraction**  
An abstraction that compiles to code equivalent to what an expert would write by hand, with no additional runtime overhead. AERO's effect system, generic types, and world bindings are all zero-cost abstractions.

---

*AERO Lang System Glossary v1.0*
