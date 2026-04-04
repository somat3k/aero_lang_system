# AERO Lang System — System Architecture

**Document Type:** Technical Architecture Specification  
**Version:** 1.0  
**Status:** Baseline

---

## Table of Contents

1. [Overview](#1-overview)
2. [High-Level Component Model](#2-high-level-component-model)
3. [Compiler Pipeline](#3-compiler-pipeline)
4. [AERO Virtual Machine (AVM)](#4-aero-virtual-machine-avm)
5. [Runtime Services](#5-runtime-services)
6. [World-Model Subsystem](#6-world-model-subsystem)
7. [Observability Pipeline](#7-observability-pipeline)
8. [Package Ecosystem](#8-package-ecosystem)
9. [Deployment Architecture](#9-deployment-architecture)
10. [Cross-Cutting Concerns](#10-cross-cutting-concerns)

---

## 1. Overview

The AERO Lang System is structured as a layered architecture where each layer has a single, clearly defined responsibility. The layers are:

```
┌─────────────────────────────────────────────────────────────┐
│                        User Programs                         │
│              (.aero source files, Aero.toml)                │
├─────────────────────────────────────────────────────────────┤
│                    AERO Standard Library                     │
│        (collections, io, net, crypto, telemetry, …)         │
├─────────────────────────────────────────────────────────────┤
│                     AERO Compiler (aeroc)                   │
│      Lexer → Parser → Type Checker → IR → Code Gen          │
├─────────────────────────────────────────────────────────────┤
│               AERO Virtual Machine (AVM)                    │
│  Scheduler │ GC │ Hot-Reload │ Reconciliation Engine        │
├─────────────────────────────────────────────────────────────┤
│             Operating System / Host Runtime                  │
│            (Linux, macOS, Windows, WASM host)               │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. High-Level Component Model

```
                        ┌──────────────┐
                        │  aeroc CLI   │  (compile, check, run, fmt, test)
                        └──────┬───────┘
                               │ produces
                               ▼
                        ┌──────────────┐
                        │  AVM Bytecode│  (.avm files, package bundles)
                        └──────┬───────┘
                               │ loaded by
                               ▼
         ┌─────────────────────────────────────────────┐
         │                  AVM Runtime                 │
         │                                             │
         │  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
         │  │Scheduler │  │   GC     │  │Hot-Reload│  │
         │  └────┬─────┘  └────┬─────┘  └────┬─────┘  │
         │       │             │              │         │
         │  ┌────▼─────────────▼──────────────▼──────┐  │
         │  │           Actor Mesh                    │  │
         │  │  (supervision trees, mailboxes, links)  │  │
         │  └────────────────┬────────────────────────┘  │
         │                   │                            │
         │  ┌────────────────▼──────────────────────┐    │
         │  │      World-Model Reconciliation        │    │
         │  │   (observe → model → delta → adapt)   │    │
         │  └────────────────┬──────────────────────┘    │
         │                   │                            │
         │  ┌────────────────▼──────────────────────┐    │
         │  │     Observability Pipeline             │    │
         │  │  (logs, metrics, traces, health)       │    │
         │  └───────────────────────────────────────┘    │
         └─────────────────────────────────────────────────┘
                               │
               ┌───────────────┼───────────────┐
               ▼               ▼               ▼
        OS Filesystem    Network/HTTP     World Adapters
        (linear caps)   (http effect)   (sensor, DB, API)
```

---

## 3. Compiler Pipeline

### 3.1 Pipeline Stages

```
Source (.aero)
     │
     ▼
┌──────────┐
│  Lexer   │  Token stream: keywords, identifiers, literals, operators
└────┬─────┘
     ▼
┌──────────┐
│  Parser  │  Concrete Syntax Tree (CST) → Abstract Syntax Tree (AST)
└────┬─────┘
     ▼
┌──────────────────┐
│  Name Resolution │  Resolve symbols, imports, module paths
└────┬─────────────┘
     ▼
┌──────────────────┐
│  Type Inference  │  Hindley-Milner + dependent type elaboration
│  & Effect Check  │  Verify effect sets; infer missing annotations
└────┬─────────────┘
     ▼
┌──────────────────┐
│  World Checker   │  Validate world type bindings and consistency models
└────┬─────────────┘
     ▼
┌──────────────────┐
│  Capability      │  Verify capability tokens flow correctly
│  Verifier        │  No capability escapes its granted scope
└────┬─────────────┘
     ▼
┌──────────────────┐
│  MIR (Mid IR)    │  Monomorphised, closure-converted, CPS-transformed
└────┬─────────────┘
     ▼
┌──────────────────┐
│  Optimiser       │  Inlining, dead-code elimination, effect fusion
└────┬─────────────┘
     ▼
┌──────────────────┐
│  Code Generator  │  AVM bytecode  OR  native via LLVM backend
└──────────────────┘
```

### 3.2 Compiler Invocation

```bash
# Type-check only (fast feedback loop)
aeroc check src/

# Compile to AVM bytecode
aeroc build --release

# Compile to native binary (LLVM backend)
aeroc build --release --target native

# Compile to WebAssembly
aeroc build --release --target wasm32
```

### 3.3 Incremental Compilation

The compiler maintains a dependency graph at the declaration level. Only declarations whose inputs have changed since the last build are re-compiled. Incremental state is stored in `target/.aeroc_cache/`.

---

## 4. AERO Virtual Machine (AVM)

### 4.1 Instruction Set Architecture

The AVM is a **register-based VM** with 256 virtual registers per stack frame. Instructions are fixed-width (32-bit) or variable-width with a one-byte opcode prefix. Key instruction categories:

| Category | Examples |
|----------|---------|
| Arithmetic | `ADD`, `SUB`, `MUL`, `DIV`, `MOD`, `POW` |
| Logic | `AND`, `OR`, `XOR`, `NOT`, `SHL`, `SHR` |
| Control Flow | `JMP`, `JMPIF`, `CALL`, `TAILCALL`, `RET` |
| Memory | `LOAD`, `STORE`, `ALLOC`, `FREE_LINEAR` |
| Actor | `SPAWN`, `SEND`, `RECV`, `LINK`, `MONITOR` |
| Effect | `EFFECT_PUSH`, `EFFECT_POP`, `EFFECT_HANDLE` |
| World | `WORLD_OBSERVE`, `WORLD_EMIT`, `WORLD_DELTA` |
| Telemetry | `LOG`, `METRIC`, `SPAN_OPEN`, `SPAN_CLOSE` |

### 4.2 Scheduler

The AVM scheduler implements a **work-stealing, M:N cooperative scheduler**:

```
OS Threads (N)
  └── Each owns a run-queue of Green Threads (actors)
         │
         ├── Active actor runs until: yield point, I/O wait, or time-slice expiry
         │
         └── Idle threads steal work from busiest peer queues
```

Yield points are inserted by the compiler at:
- Back-edges of loops (preventing starvation),
- All I/O and effect boundary crossings,
- Explicit `yield` expressions.

### 4.3 Garbage Collector

The AVM GC is a **tri-colour, incremental, generational collector**:

| Generation | Description | Collection Trigger |
|------------|-------------|-------------------|
| Nursery (Gen0) | Short-lived allocations; bump-pointer allocator | Every ~256 KB |
| Minor (Gen1) | Survived Gen0; semi-space collector | Every ~4 MB |
| Major (Gen2) | Long-lived objects; mark-compact | Every ~64 MB or on demand |

Linear-typed values are freed deterministically when they go out of scope (no GC involvement), eliminating a major source of GC pressure in resource-heavy programs.

### 4.4 Hot-Code Replacement

The AVM supports loading a new version of a module while the program is running:

1. New bytecode is verified (type-safe, capability-safe) by the verifier.
2. New module is loaded alongside the old one.
3. Actors that subscribe to hot-reload events receive `ModuleUpdated` messages.
4. Actors migrate to the new module on their next scheduled turn.
5. Old module bytecode is unloaded once no actors reference it.

This mechanism is used for zero-downtime deployments, live configuration changes, and runtime A/B testing.

---

## 5. Runtime Services

### 5.1 HTTP Runtime (`aero-http`)

Provides an async HTTP/1.1 and HTTP/2 server and client built on the AVM scheduler:

```aero
use aero_http::{Server, Request, Response};

fn handle(req: Request) -> Response ! [log] {
    emit log::info("request received", { path: req.path, method: req.method });
    Response::ok().body("Hello, AERO!")
}

fn main() ! [http] {
    Server::bind("0.0.0.0:8080")
        .handler(handle)
        .serve()
}
```

### 5.2 Database Runtime (`aero-db`)

Provides typed, capability-gated access to relational and document databases with:
- Connection pooling managed as a supervised actor pool,
- Query builders that respect data classification annotations,
- Automatic metric emission for query latency and error rates.

### 5.3 Telemetry Runtime (`aero-telemetry`)

Implements the AERO Observability Pipeline (see §7) and exports to:
- **OpenTelemetry Collector** (OTLP gRPC/HTTP),
- **Prometheus** (scrape endpoint),
- **Structured stdout** (development mode).

---

## 6. World-Model Subsystem

### 6.1 World Adapter Interface

Every world type must have a corresponding **World Adapter** — a module implementing the `WorldAdapter` interface:

```aero
interface WorldAdapter<W: World> {
    fn observe(&self) -> W ! [io];
    fn emit(&self, delta: W::Delta) -> Result<(), AdapterError> ! [io];
    fn consistency_model() -> ConsistencyModel;
}
```

Adapters are registered at startup and bound to world types by name. Built-in adapters are provided for:

| Adapter | World Type | Description |
|---------|-----------|-------------|
| `HttpJsonAdapter` | Any `world` with `#[serde]` | Poll a JSON REST endpoint |
| `KafkaAdapter` | Any `world` | Consume/produce Kafka topics |
| `GrpcAdapter` | Any `world` with `#[proto]` | Bidirectional gRPC streaming |
| `SqlAdapter` | Any `world` with `#[sql]` | Watch a SQL table/view |
| `FileAdapter` | Any `world` with `#[csv]` or `#[json]` | Watch a filesystem path |
| `MockAdapter` | Any `world` | Scripted test adapter |

### 6.2 Reconciliation Algorithm

```
Every reconciliation_interval (default: 1s):
  For each active world binding W:
    1. Call adapter.observe() → current_state
    2. Compute delta = diff(last_known_state, current_state)
    3. If delta is non-empty:
       a. Update last_known_state
       b. Classify drift severity (Minor | Major | Critical)
       c. Dispatch WorldChanged(W, delta, severity) to all subscribers
       d. Emit telemetry event world.reconciliation.delta
    4. If adapter.observe() fails:
       a. Increment failure counter
       b. If counter > threshold: dispatch WorldUnavailable(W)
       c. Apply circuit-breaker to adapter
```

---

## 7. Observability Pipeline

```
Program                  AVM Runtime               Export Backends
  │                          │
  ├── emit log::info(…)  ──► Log Buffer (ring) ──► OTLP Collector
  │                          │                        │
  ├── emit log::metric(…) ─► Metric Aggregator ───► Prometheus
  │                          │                        │
  ├── (actor message)    ──► Trace Context Prop. ──► OTLP Collector
  │                          │
  └── (supervision event) ─► Health Model ─────────► /health endpoint
```

All telemetry is non-blocking: the AVM buffers events in a lock-free ring buffer and drains them on a dedicated I/O actor. If the export backend is unavailable, events are held in the buffer until space is exhausted, at which point they are dropped with a counter increment (never blocking the program).

---

## 8. Package Ecosystem

### 8.1 Directory Structure

A typical AERO package:

```
my-service/
├── Aero.toml          # Package manifest
├── Aero.lock          # Dependency lockfile (committed to VCS)
├── src/
│   ├── main.aero      # Entry point
│   ├── lib.aero       # Library root (if dual crate)
│   └── models/
│       └── user.aero
├── tests/
│   └── integration.aero
├── benchmarks/
│   └── throughput.aero
└── docs/
    └── README.md
```

### 8.2 Manifest Schema (`Aero.toml`)

```toml
[package]
name        = "my-service"
version     = "0.1.0"
edition     = "2026"
authors     = ["Your Name <you@example.com>"]
description = "A brief description of the package"
license     = "MIT OR Apache-2.0"

[capabilities]
# Declare capabilities this package may request at runtime
network = ["outbound:https"]
filesystem = ["read:/etc/config"]

[dependencies]
aero-http      = { version = "1.2", features = ["http2"] }
aero-telemetry = "2.0"

[dev-dependencies]
aero-test = "1.0"

[profile.release]
opt-level    = 3
lto          = true
strip        = "symbols"
```

---

## 9. Deployment Architecture

### 9.1 Single-Process Deployment

For smaller services, the entire AERO program runs as a single AVM process. The supervision tree provides internal resilience; external resilience is provided by the container orchestrator (Kubernetes, Nomad, etc.).

### 9.2 Distributed Deployment

For systems requiring horizontal scaling, multiple AVM processes connect via the **AERO Cluster Protocol** — a binary protocol built on top of TLS mutual authentication. The actor addressing model is cluster-aware: `ActorRef<T>` can transparently reference actors in remote nodes.

```
┌──────────────────┐       ┌──────────────────┐
│   AVM Node A     │◄─────►│   AVM Node B     │
│  (actors 1…N)    │ Cluster│  (actors N+1…M)  │
│                  │ Protocol│                  │
└──────────────────┘       └──────────────────┘
         │                          │
         └──────────┬───────────────┘
                    ▼
            Service Registry
          (etcd / Consul / built-in)
```

### 9.3 Container Image

The AERO toolchain produces a minimal OCI container image:

```bash
aeroc build --release --target native
aeroc package --oci --base scratch --output my-service.tar
```

Typical image size: **~8 MB** (static binary + AVM runtime, no OS dependencies).

---

## 10. Cross-Cutting Concerns

### 10.1 Logging Configuration

Log levels and output format are configured via environment variables at runtime:

| Variable | Default | Description |
|----------|---------|-------------|
| `AERO_LOG` | `info` | Minimum log level (`trace`, `debug`, `info`, `warn`, `error`) |
| `AERO_LOG_FORMAT` | `json` | Log format (`json`, `text`, `pretty`) |
| `AERO_LOG_OUTPUT` | `stdout` | Log sink (`stdout`, `stderr`, `file:/path`) |

### 10.2 Feature Flags

Runtime feature flags integrate with the world-model system — they are modelled as a `world FlagState` binding, allowing flag changes to propagate to running actors in real time without restart.

### 10.3 Configuration Management

Configuration is loaded at startup from:
1. `config/default.toml` (compiled in as default values),
2. `config/{AERO_ENV}.toml` (environment-specific overrides),
3. Environment variables (highest precedence).

All configuration values are typed and validated against a declared schema at startup; the program will not start if configuration is invalid.

---

*AERO Lang System Architecture Specification v1.0*
