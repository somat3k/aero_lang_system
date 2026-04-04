# AERO Lang System — Documentation Hub

> **FutureProof AERO Buff** — A next-generation language and runtime framework designed to bridge elegant code with real-world living systems.

---

## Document Index

| # | Document | Description |
|---|----------|-------------|
| 1 | [Whitepaper](./whitepaper.md) | Enterprise academic whitepaper — vision, theory, and formal specification |
| 2 | [Architecture](./architecture.md) | Complete system architecture, component model, and data-flow diagrams |
| 3 | [Design Principles](./design_principles.md) | Core philosophy, language tenets, and coding conventions |
| 4 | [Getting Started](./getting_started.md) | Installation, first program, and guided project walkthrough |
| 5 | [API Reference](./api_reference.md) | Comprehensive language & runtime API reference |
| 6 | [Roadmap](./roadmap.md) | Phased development plan, milestones, and delivery schedule |
| 7 | [Glossary](./glossary.md) | Terminology, acronyms, and concept definitions |

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
Whitepaper  →  Architecture  →  Design Principles
     ↓               ↓
Getting Started  →  API Reference
     ↓
   Roadmap  →  Glossary
```

New contributors should start with the **Whitepaper** to understand the motivation, then proceed to **Architecture** and **Design Principles** before writing any code.

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
