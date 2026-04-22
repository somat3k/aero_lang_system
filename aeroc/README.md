# AEROC - AERO Language Compiler & Toolchain

> The official Rust-based compiler and toolchain for the AERO programming language

## Overview

`aeroc` is the complete toolchain for developing AERO programs. It provides a comprehensive CLI for creating, building, running, and testing AERO applications.

AERO (Adaptive, Efficient, Resilient, Observable) is a next-generation programming language where adaptive, observable, and resilient software is the natural default.

## Features

- **Project Scaffolding** - `aeroc new` creates fully-configured AERO projects
- **Type Checking** - `aeroc check` validates your code without compilation
- **Building** - `aeroc build` compiles AERO source to AVM bytecode
- **Running** - `aeroc run` executes your AERO programs
- **Testing** - `aeroc test` runs tests marked with `#[test]`
- **Formatting** - `aeroc fmt` formats code according to AERO style

## Architecture

The AEROC toolchain is composed of several modular libraries:

```
aeroc/                    # Main CLI binary
├── aero-lexer/          # Lexical analysis (tokenization)
├── aero-parser/         # Syntax analysis (AST generation)
├── aero-ast/            # Abstract Syntax Tree definitions
├── aero-types/          # Type checking and effect system
├── aero-codegen/        # Code generation to AVM bytecode
└── avm-runtime/         # AERO Virtual Machine runtime
```

## Installation

### Building from Source

```bash
# Clone the repository
git clone https://github.com/somat3k/aero_lang_system.git
cd aero_lang_system

# Build the toolchain
cargo build --release

# Install globally (optional)
cargo install --path aeroc
```

## Quick Start

### Create a New Project

```bash
aeroc new my-project
cd my-project
```

This creates a new AERO project with the following structure:

```
my-project/
├── Aero.toml           # Package manifest
├── .gitignore
└── src/
    └── main.aero       # Entry point
```

### Run Your Program

```bash
aeroc run
```

### Check for Errors

```bash
aeroc check
```

### Build for Release

```bash
aeroc build --release
```

### Run Tests

```bash
aeroc test
```

### Format Code

```bash
aeroc fmt
```

## AERO Language Features

### The `know` Keyword

AERO uses `know` instead of traditional variable binding to assert knowledge:

```aero
fn main() {
    know x = 42;
    know greeting = "Hello, AERO!";
}
```

### Effect System

Functions declare their side effects explicitly:

```aero
fn process_data() ! [log, http, db] {
    emit log::info("Processing data");
    emit http::get("https://api.example.com/data");
    emit db::save(data);
}
```

### World Types

Model real-world knowledge as first-class types:

```aero
world Temperature {
    adapter = SensorAdapter,
    poll_interval = "5s",
}

fn monitor() ! [log, world] {
    know temp = world::Temperature::observe();
    emit log::info("Current temperature: {}", temp);
}
```

### Pattern Matching

```aero
fn process(value: Option<i64>) -> i64 {
    match value {
        Some(x) => x * 2,
        None => 0,
    }
}
```

## Project Configuration

The `Aero.toml` manifest configures your project:

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2026"

[capabilities]
network = ["outbound:https"]

[dependencies]
aero-http = "1.2"

[world.temperature]
adapter = "SensorAdapter"
url = "tcp://sensor.local:8080"
poll_interval = "5s"
```

## CLI Commands

### `aeroc new <name>`

Create a new AERO project

Options:
- `--path <path>` - Specify project directory

### `aeroc check`

Type-check the project without building

Options:
- `--path <path>` - Project directory (default: `.`)

### `aeroc build`

Compile the project to AVM bytecode

Options:
- `--path <path>` - Project directory (default: `.`)
- `--release` - Build with optimizations

### `aeroc run`

Build and execute the project

Options:
- `--path <path>` - Project directory (default: `.`)
- `--release` - Run optimized build
- `<args>...` - Arguments to pass to the program

### `aeroc test`

Run project tests

Options:
- `--path <path>` - Project directory (default: `.`)
- `<filter>` - Filter tests by name

### `aeroc fmt`

Format AERO source files

Options:
- `--path <path>` - Project directory (default: `.`)
- `--check` - Check formatting without modifying files

## Development Status

This is the v0.1 foundation release (Q3 2026). The toolchain currently includes:

- ✅ Full CLI with all subcommands
- ✅ Lexer with complete AERO syntax support
- ✅ Parser generating full AST
- ✅ Basic type system foundations
- ✅ AVM bytecode generation (placeholder)
- ✅ VM runtime (placeholder execution)

### Roadmap

**v0.2 (Q4 2026)** - Language Core
- Full effect system implementation
- Effect handlers
- World types with adapters
- Linear types
- Pattern matching with exhaustiveness
- Module system

**v0.3 (Q1 2027)** - Runtime Hardening
- Generational GC
- Work-stealing scheduler
- Actor supervision
- Hot-code replacement
- Reconciliation engine

**v0.4 (Q2 2027)** - Ecosystem
- Package registry
- LSP (Language Server Protocol)
- WASM target
- Database adapters

## Contributing

Contributions are welcome! See the main [repository](https://github.com/somat3k/aero_lang_system) for contribution guidelines.

Key areas for contribution:
- Compiler optimization
- Standard library implementation
- Test coverage
- Documentation
- Error messages

## Architecture Details

### Compilation Pipeline

```
Source (.aero)
    ↓
aero-lexer (Tokenization)
    ↓
aero-parser (AST Generation)
    ↓
aero-types (Type Checking + Effect Analysis)
    ↓
aero-codegen (Bytecode Generation)
    ↓
AVM Bytecode (.avm)
    ↓
avm-runtime (Execution)
```

### Lexer (aero-lexer)

Built with [Logos](https://github.com/maciejhirsz/logos), the lexer tokenizes AERO source:

- Keywords: `fn`, `know`, `emit`, `struct`, `enum`, `world`, `match`, etc.
- Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `!`, `?`
- Literals: strings, integers, floats, booleans
- Identifiers and symbols

### Parser (aero-parser)

Recursive descent parser generating a fully-typed AST:

- Functions with effect declarations
- Structs, enums, and world types
- Pattern matching
- Effect emissions (`emit`)
- Knowledge assertions (`know`)

### Type System (aero-types)

Effect-aware type checker:

```
Γ ⊢ e : τ ! ε
```

Where:
- `Γ` = typing context
- `e` = expression
- `τ` = value type
- `ε` = effect set

### Code Generation (aero-codegen)

Generates register-based bytecode for the AVM:

- Optimization passes (in development)
- Effect lowering
- Linear type enforcement

### Runtime (avm-runtime)

Register-based virtual machine:

- Stack-based execution
- GC integration (v0.3+)
- Effect dispatch
- Observability hooks

## License

Dual-licensed under MIT or Apache-2.0, at your option.

## Links

- [Documentation](https://docs.aero-lang.dev)
- [GitHub Repository](https://github.com/somat3k/aero_lang_system)
- [Whitepaper](../docs/whitepaper.md)
- [Language Identity](../docs/language_identity.md)
- [Roadmap](../docs/roadmap.md)

---

**AERO Lang System** — Making adaptive, resilient, and observable software the natural default.
