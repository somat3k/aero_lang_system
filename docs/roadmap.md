# AERO Lang System — Development Roadmap

**Document Type:** Product Roadmap  
**Version:** 1.0  
**Status:** Active

---

## Vision

> Build a programming language and runtime that makes adaptive, observable, and resilient software the natural default — not an engineering heroic effort.

Every milestone below advances that vision by making AERO more expressive, more correct, and more useful in real enterprise environments.

---

## Release Overview

| Milestone | Name | Target | Status |
|-----------|------|--------|--------|
| v0.1 | Foundation | Q3 2026 | 🔵 In Progress |
| v0.2 | Language Core | Q4 2026 | ⬜ Planned |
| v0.3 | Runtime Hardening | Q1 2027 | ⬜ Planned |
| v0.4 | Ecosystem Build-out | Q2 2027 | ⬜ Planned |
| v1.0 | General Availability | Q3 2027 | ⬜ Planned |
| v1.x | Post-GA Iteration | Ongoing | ⬜ Planned |

---

## Milestone Detail

### v0.1 — Foundation (Q3 2026)

**Goal:** Establish the core toolchain so that the team can write, compile, and run basic AERO programs.

| # | Deliverable | Description |
|---|-------------|-------------|
| F-01 | `aeroc` compiler CLI | `check`, `build`, `run`, `fmt` subcommands |
| F-02 | Lexer & Parser | Full AERO 2026 grammar, error recovery |
| F-03 | Basic type system | Primitives, structs, enums, generics (no effects yet) |
| F-04 | AVM v0.1 | Register-based bytecode interpreter, no GC |
| F-05 | Standard library skeleton | `std::collections`, `std::string`, `std::io` |
| F-06 | Test runner | `aeroc test` with `#[test]` attribute |
| F-07 | `docs/` folder | Initial documentation (this set of files) |

**Definition of Done:** A developer can `aeroc new`, write a struct-and-function program, compile it, and run it in the AVM. Tests pass.

---

### v0.2 — Language Core (Q4 2026)

**Goal:** Implement the effect system, world types, and linear types — the features that differentiate AERO from existing languages.

| # | Deliverable | Description |
|---|-------------|-------------|
| L-01 | Effect types | `! [effects]` syntax, effect inference, effect propagation |
| L-02 | Effect handlers | `handle … with …` syntax and handler dispatch |
| L-03 | World types | `world` declarations, world adapter interface |
| L-04 | Linear types | Linear type checker; `FREE_LINEAR` bytecode instruction |
| L-05 | `Result` & `Option` | Compiler-native result/option types with `?` operator |
| L-06 | Pattern matching | `match` expressions with exhaustiveness checking |
| L-07 | Module system | `use`, `pub`, `mod`, package-level imports |
| L-08 | Capability declarations | `[capabilities]` in `Aero.toml`, capability tokens |
| L-09 | Telemetry API | `emit log::*`, `emit log::metric`, auto span injection |
| L-10 | Improved stdlib | `std::time`, `std::net` (TCP/UDP), `std::fs` |

**Definition of Done:** The temperature monitor example from the Getting Started guide compiles and runs correctly with a mock adapter.

---

### v0.3 — Runtime Hardening (Q1 2027)

**Goal:** Production-grade runtime: GC, actor supervision, hot-reload, reconciliation engine.

| # | Deliverable | Description |
|---|-------------|-------------|
| R-01 | Generational GC | Tri-colour incremental GC (Gen0, Gen1, Gen2) |
| R-02 | Work-stealing scheduler | M:N cooperative scheduler with yield-point injection |
| R-03 | Supervision trees | `supervisor` declarations, restart strategies |
| R-04 | Hot-code replacement | Module reload without process restart |
| R-05 | Reconciliation engine | Background world-model drift detection and dispatch |
| R-06 | `aero-http` v1 | Async HTTP/1.1 and HTTP/2 server and client |
| R-07 | `aero-telemetry` v1 | OTLP export, Prometheus scrape endpoint |
| R-08 | Health API | `/health` endpoint from supervision tree state |
| R-09 | Native backend | LLVM code generator (native binary target) |
| R-10 | Benchmark suite | Throughput and latency benchmarks for AVM and GC |

**Definition of Done:** An AERO HTTP service can handle 10 000 req/s on a 2-core machine with P99 latency < 5 ms, measured by the benchmark suite.

---

### v0.4 — Ecosystem Build-out (Q2 2027)

**Goal:** Build the package ecosystem and developer experience infrastructure required for real-world adoption.

| # | Deliverable | Description |
|---|-------------|-------------|
| E-01 | Package registry | Public registry at `pkg.aero-lang.dev` |
| E-02 | `aero-db` v1 | PostgreSQL and SQLite world adapters |
| E-03 | `aero-kafka` v1 | Apache Kafka world adapter |
| E-04 | WASM target | Compile AERO to WebAssembly for edge/browser deployment |
| E-05 | Language server (LSP) | VS Code and Neovim integration: completion, diagnostics, hover |
| E-06 | Formatter (`aerofmt`) | Opinionated, deterministic code formatter |
| E-07 | Linter (`aeroclip`) | Style, correctness, and security lint rules |
| E-08 | Cluster protocol v1 | Multi-node AVM deployment, remote ActorRef |
| E-09 | Data classification | `@sensitive` annotation, redaction enforcement |
| E-10 | Documentation site | `docs.aero-lang.dev` — searchable, versioned |

**Definition of Done:** An external contributor can publish a package to the registry and have it consumed by a third project using only public tooling.

---

### v1.0 — General Availability (Q3 2027)

**Goal:** Declare the language specification and public APIs stable. Commit to semantic versioning guarantees.

| # | Deliverable | Description |
|---|-------------|-------------|
| G-01 | Language specification v1.0 | Formal grammar, type rules, execution semantics |
| G-02 | Stability guarantee | No breaking changes in `v1.x` without 12-month deprecation |
| G-03 | Enterprise security audit | Third-party security audit of compiler, AVM, and stdlib |
| G-04 | Migration guide | Guide for developers coming from Rust, Go, Java, Python |
| G-05 | Production case study | At least one production deployment documented and public |
| G-06 | Community governance | RFC process, code of conduct, contributor guidelines |
| G-07 | Long-term support policy | LTS release cadence and support commitment |

**Definition of Done:** Language specification is published; the stability guarantee is publicly committed; the security audit report is published.

---

### v1.x — Post-GA Iteration (Ongoing)

Areas of active research and development following v1.0 GA:

| Area | Description |
|------|-------------|
| **Verified Compilation** | Integration with formal verification tools for critical components |
| **Dependent Type Extensions** | Richer dependent types for data shape and range constraints |
| **AI-Assisted Reconciliation** | Machine-learning-based anomaly detection in the reconciliation engine |
| **Distributed Transactions** | First-class two-phase commit and saga patterns for world emits |
| **AERO on Bare Metal** | Embedded and RTOS targets for IoT and edge hardware |
| **Gradual Adoption Path** | FFI and interop layers for calling AERO from Rust, Go, and Python |

---

## Guiding Principles for Roadmap Decisions

1. **Correctness before features.** A smaller, correct system is preferred over a larger, buggy one.
2. **Dogfood first.** AERO tooling (compiler, package manager, registry) must be implemented in AERO itself as early as possible.
3. **Community input.** Major language changes go through the public RFC process before implementation begins.
4. **Stability matters.** No breaking changes in released minor versions. Deprecations are announced at least one release cycle in advance.

---

## Contributing

A formal `CONTRIBUTING.md` guide is planned for v0.3. Until then, use GitHub Discussions for contribution questions and roadmap proposals:
https://github.com/somat3k/aero_lang_system/discussions

---

*AERO Lang System Roadmap v1.0 — last updated 2026-04-04*
