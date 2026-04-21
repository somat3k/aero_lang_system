# AERO Lang System

> **FutureProof AERO Buff** — A next-generation programming language and runtime framework where adaptive, resilient, and observable software is the natural default.

**AERO** — *Adaptive, Efficient, Resilient, Observable* — closes the gap between static code and the dynamic, living systems it serves.

---

## Documentation

All project documentation lives in the [`docs/`](./docs/) folder:

| Document | Description |
|----------|-------------|
| [📄 Whitepaper](./docs/whitepaper.md) | Enterprise academic whitepaper — vision, theory, formal specification |
| [🧬 Language Identity](./docs/language_identity.md) | AERO's distinct syntax, `know` keyword, knowledge-surface model, autonomy, micro-environments |
| [🔮 HoloLang](./docs/hololang.md) | Domain bridge DSL for holographic / physical-device systems: DDAC, GEMM, SafeTensor, MDI canvas |
| [🖥 Systems Stack](./docs/systems_stack.md) | Full-stack blueprint: hardware → OS → network → runtime → application |
| [⚙️ OS Design](./docs/os_design.md) | AeroOS — programming-oriented, capability-based operating system |
| [🌐 Network Architecture](./docs/network_architecture.md) | Lean, programmable network layer: data / control / management planes |
| [🏗 Architecture](./docs/architecture.md) | AERO runtime: compiler pipeline, AVM, scheduler, GC, observability |
| [📐 Design Principles](./docs/design_principles.md) | Core philosophy, language tenets, and coding conventions |
| [🚀 Getting Started](./docs/getting_started.md) | Installation, first program, and guided project walkthrough |
| [📚 API Reference](./docs/api_reference.md) | Comprehensive language and runtime API reference |
| [🗺 Roadmap](./docs/roadmap.md) | Phased development plan, milestones, and delivery schedule |
| [📖 Glossary](./docs/glossary.md) | Terminology, acronyms, and concept definitions |

Start with the **[Whitepaper](./docs/whitepaper.md)** for the full vision, then **[Language Identity](./docs/language_identity.md)** to understand AERO's distinct philosophy. For physical device / holographic systems, read **[HoloLang](./docs/hololang.md)**.

---

## Implementation Status

**v0.1 Foundation (Q3 2026) — ✅ IN PROGRESS**

The AERO compiler toolchain is now available as an independent Rust package:

```
aero_lang_system/
├── aeroc/               # Main CLI compiler
├── aero-lexer/          # Tokenization
├── aero-parser/         # Syntax analysis
├── aero-ast/            # Abstract Syntax Tree
├── aero-types/          # Type checking
├── aero-codegen/        # Bytecode generation
└── avm-runtime/         # Virtual machine runtime
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/somat3k/aero_lang_system.git
cd aero_lang_system

# Build the toolchain
cargo build --release

# The aeroc binary is now available at:
# ./target/release/aeroc
```

### What's Working Now

- ✅ Full CLI with all v0.1 subcommands (`new`, `check`, `build`, `run`, `test`, `fmt`)
- ✅ Lexer with complete AERO syntax tokenization
- ✅ Parser generating full AST with effect declarations
- ✅ Basic type checking infrastructure
- ✅ AVM bytecode generation (foundation)
- ✅ VM runtime with placeholder execution
- ✅ Project scaffolding with `aeroc new`
- ✅ All unit tests passing

See the [aeroc README](./aeroc/README.md) for detailed CLI usage and development guide.

---

## Quick Start

```bash
# Install the toolchain
curl -fsSL https://get.aero-lang.dev | sh

# Create a new project
aeroc new my-project && cd my-project

# Run it
aeroc run
```

---

## Contributing

Contributions welcome! See the [Roadmap](./docs/roadmap.md) for planned milestones and open areas of work.  
Open an [Issue](https://github.com/somat3k/aero_lang_system/issues) or start a [Discussion](https://github.com/somat3k/aero_lang_system/discussions).
