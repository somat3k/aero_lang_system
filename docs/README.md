# AERO Lang System — Documentation Hub

> **FutureProof AERO Buff** — A next-generation language and runtime framework designed to bridge elegant code with real-world living systems.

---

## Document Index

| # | Document | Description |
|---|----------|-------------|
| 1 | [Whitepaper](./whitepaper.md) | Enterprise academic whitepaper — vision, theory, and formal specification |
| 2 | [Language Identity](./language_identity.md) | AERO's distinct syntax, knowledge-surface model, autonomy, and micro-environments |
| 3 | [Systems Stack](./systems_stack.md) | Full-stack blueprint: hardware → OS → network → runtime → application |
| 4 | [OS Design](./os_design.md) | AeroOS — programming-oriented, capability-based operating system |
| 5 | [Network Architecture](./network_architecture.md) | Lean, programmable network layer: data / control / management planes |
| 6 | [Architecture](./architecture.md) | AERO runtime architecture, component model, and data-flow diagrams |
| 7 | [Design Principles](./design_principles.md) | Core philosophy, language tenets, and coding conventions |
| 8 | [Getting Started](./getting_started.md) | Installation, first program, and guided project walkthrough |
| 9 | [API Reference](./api_reference.md) | Comprehensive language & runtime API reference |
| 10 | [Roadmap](./roadmap.md) | Phased development plan, milestones, and delivery schedule |
| 11 | [Glossary](./glossary.md) | Terminology, acronyms, and concept definitions |

---

## What Is AERO?

**AERO** — *Adaptive, Efficient, Resilient, Observable* — is a programming language and runtime system built on four guiding forces:

- **Adaptive** — Programs that learn and adjust to their environment at runtime without redeployment.
- **Efficient** — Zero-cost abstractions; the compiler optimises away every layer of indirection.
- **Resilient** — Fault-tolerance baked into the type system; errors are values, not exceptions.
- **Observable** — First-class tracing, metrics, and structured logging woven into the language itself.

AERO is not just a language — it is a *living system specification*: every program written in it maintains an explicit model of the world it operates in and can react to changes in that world in real time.

---

## Reading Order

```
Whitepaper → Language Identity → Systems Stack → OS Design → Network Architecture
     ↓               ↓
 Architecture → Design Principles
     ↓
Getting Started → API Reference
     ↓
   Roadmap → Glossary
```

New contributors should start with the **Whitepaper** for the vision and motivation, then **Language Identity** to understand AERO's distinct syntax and philosophy (read this *before* writing any AERO code), then **Systems Stack** for the full hardware-to-application blueprint.

---

## Contributing to Documentation

All documentation lives in the `docs/` directory as Markdown files.  
They can be rendered as PDFs using any standard Markdown-to-PDF pipeline (e.g., `pandoc`, `mdpdf`).

```bash
# Example: generate PDF from whitepaper
pandoc docs/whitepaper.md -o docs/whitepaper.pdf --pdf-engine=xelatex
```

---

*AERO Lang System © 2026. All rights reserved.*
