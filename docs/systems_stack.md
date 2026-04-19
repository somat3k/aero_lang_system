# AERO Systems Stack — Full-Stack Blueprint

**Document Type:** Systems Architecture Blueprint  
**Version:** 1.0  
**Scope:** Hardware → OS → Network → Runtime → Application

---

## Introduction

The AERO Lang System does not live in isolation. It is designed as the **top layer of a coherent, programmable, lean systems stack** — one where every layer from bare metal to application shares the same design contract: observable by default, failure as a first-class input, declarative state, and a small trusted core.

This document describes how to assemble that stack. It answers the question: *"If you were building a computer and network from scratch, how would you do it — lean, robust, and programming-oriented?"*

The answer is not to redesign semiconductor fabs. It is to **own the interfaces** and make every layer programmable end-to-end.

---

## The Four Levels of "From Scratch"

Before choosing where to start, the team must be clear about what "from scratch" means at each level:

| Level | Description | Capital / Time | Recommended Starting Point |
|-------|-------------|----------------|-----------------------------|
| **L1** | Commodity x86-64/ARM64 hardware + fully custom software stack | Low | ✅ Start here |
| **L2** | Standard chips, custom PCB/appliance/network board design | Medium | ✅ Grow into this |
| **L3** | Custom ISA, processor, NIC, or accelerator | High | ⚠️ Later — needs revenue |
| **L4** | Semiconductor fabrication | Extreme | ❌ Not lean |

**The lean strategy is L1 now, L2 when the design is proven.** The hardware abstraction layer (HAL) in the AERO OS is designed so that switching from L1 to L2 boards requires only a HAL driver replacement, not a redesign of the OS or runtime.

---

## The Unified Stack

```
┌──────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                          │
│            AERO Programs (user services, tools)              │
├──────────────────────────────────────────────────────────────┤
│                    RUNTIME LAYER                              │
│       AERO Virtual Machine (AVM) + Standard Library          │
│   Scheduler │ GC │ World-Model Reconciliation │ Telemetry    │
├──────────────────────────────────────────────────────────────┤
│                    NETWORK LAYER                              │
│   AERO Network Plane: programmable data / control / mgmt     │
│   eBPF fast-path │ DPDK/XDP │ P4 forwarding (L2 boards)      │
├──────────────────────────────────────────────────────────────┤
│                   OPERATING SYSTEM LAYER                      │
│            AeroOS — unikernel-inspired, AERO-native          │
│   Capability-based scheduler │ Typed IPC │ Live-reload       │
├──────────────────────────────────────────────────────────────┤
│                    HARDWARE LAYER                             │
│   Commodity x86-64 / ARM64 servers and embedded boards       │
│   TPM 2.0 │ SR-IOV NIC │ NVMe │ IOMMU │ Secure Boot         │
└──────────────────────────────────────────────────────────────┘
```

Each layer communicates only with the layer immediately above and below it, through a **typed, capability-gated interface**. There is no "reach-around" from the application layer to the hardware layer — all resource access flows through the capability system.

---

## Layer 1 — Hardware

### Selection Criteria

For L1 deployment, hardware is selected for:

| Property | Requirement | Why |
|----------|-------------|-----|
| **Architecture** | x86-64 (primary), ARM64 (secondary) | Broadest toolchain support |
| **Virtualisation** | VT-x / AMD-V, IOMMU | Secure isolation, SR-IOV |
| **Security** | TPM 2.0, Secure Boot, memory encryption | Root of trust chain |
| **NIC** | SR-IOV capable, programmable (e.g., Mellanox/Intel E810) | Kernel-bypass networking |
| **Storage** | NVMe with end-to-end protection | Low-latency, checksummed I/O |
| **Memory** | ECC RAM | Silent corruption prevention |
| **Management** | IPMI/BMC, UEFI | Out-of-band management |

### Hardware Abstraction Layer (HAL)

The AeroOS HAL provides typed AERO interfaces to all hardware:

```aero
// Hardware capability tokens — granted at boot, not at will
interface CpuCap   { fn core_count() -> u32; fn frequency_mhz() -> u32; }
interface MemoryCap { fn total_bytes() -> u64; fn alloc(n: usize) -> LinearPtr ! [memory]; }
interface NicCap   { fn send(frame: EtherFrame) -> () ! [net]; fn recv() -> EtherFrame ! [net]; }
interface StorageCap { fn read(lba: u64, buf: &mut [u8]) -> Result<(), IoError> ! [storage]; }
```

The rest of the OS and runtime never calls hardware directly — it holds capability tokens.

### Secure Boot Chain

```
UEFI Secure Boot
     │  verifies signature of
     ▼
AeroOS bootloader (signed by AERO key)
     │  verifies signature of
     ▼
AeroOS kernel image (TPM PCR extended)
     │  verifies signatures of
     ▼
AVM + Standard Library bundle (measured boot)
     │
     ▼
User program (capability grants issued)
```

Every step in the chain is measured into the TPM. A remote-attestation endpoint allows operators to verify the exact software running on a node before trusting it with secrets.

---

## Layer 2 — Operating System (AeroOS)

*Full specification: [os_design.md](./os_design.md)*

AeroOS is a **programming-oriented, unikernel-inspired operating system** designed to host the AVM. Key properties:

- **No POSIX compatibility layer by default.** Applications are AERO programs; there is no shell, no `/proc`, no `fork`. This eliminates an enormous attack surface.
- **Capability-based access control** at the syscall level, mirroring the AERO language capability model.
- **Typed IPC** — inter-process communication uses the same typed message-passing model as AERO actors.
- **Live-reload** — the OS supports hot-swap of AVM and user program modules without rebooting.
- **Small trusted computing base (TCB)** — the kernel is ≤ 50 000 lines of verified AERO/C code, formally audited.

### Process Model

AeroOS uses a **single-address-space model** for trusted AVM programs (fast, no TLB shootdown) and a **hardware-isolated sandbox** for untrusted or multi-tenant workloads (IOMMU + memory encryption).

---

## Layer 3 — Network

*Full specification: [network_architecture.md](./network_architecture.md)*

The AERO network layer is built on the principle that **the network is programmable infrastructure, not a fixed black box**. It has three planes:

| Plane | Technology | Responsibility |
|-------|-----------|----------------|
| **Data plane** | eBPF/XDP (L1) or P4 (L2 custom NIC) | Forward packets at line rate |
| **Control plane** | AeroOS daemon, AERO actor mesh | Topology, routing, policy decisions |
| **Management plane** | AERO world-type bindings, gRPC API | Configuration, telemetry, automation |

The split between planes is **always explicit** — no function spans planes without a type-level boundary.

---

## Layer 4 — Runtime (AVM)

*Full specification: [architecture.md](./architecture.md)*

The AVM is the first-class citizen of the stack. It is not a guest inside a general-purpose OS — it is the **primary workload the OS is optimised to host**.

The OS exposes its primitives (threads, memory, network I/O) through capability tokens, and the AVM consumes them directly, without POSIX translation layers.

---

## Layer 5 — Application

Applications are AERO programs with declared capabilities, world types, and effect sets. They are fully encapsulated: one program cannot touch another's memory, capabilities, or world bindings without an explicit, typed interface.

---

## Design Axioms Across All Layers

These axioms apply from hardware to application:

| # | Axiom |
|---|-------|
| A1 | **Every resource access requires a capability token.** No ambient authority at any layer. |
| A2 | **State is declared, not implicit.** Configuration, topology, and policy are declarative values, not procedural mutations. |
| A3 | **Failure is a first-class input.** Every layer exposes a typed failure model. Layers above do not pretend the layer below is infallible. |
| A4 | **Observability is intrinsic.** Every layer emits structured telemetry that feeds into the AERO Observability Pipeline. There is no "silent layer." |
| A5 | **Interfaces are stable; implementations are replaceable.** The HAL interface, the OS syscall interface, the AVM instruction set, and the AERO language API all follow SemVer. Implementations evolve freely behind them. |
| A6 | **The trusted core is small and audited.** Each layer has a minimal, formally-verified core. Extensions live outside the TCB. |

---

## Evolution Path

```
Phase 1 (Now)        Phase 2               Phase 3
─────────────────    ────────────────────   ──────────────────────
Commodity servers    Custom appliance        Custom NIC / SmartNIC
COTS NICs            boards (L2)             P4 forwarding
AeroOS on Linux      AeroOS bare-metal       AeroOS on custom HW
AERO userspace       AERO data plane         AERO everywhere
```

At each phase transition, the **only changes required** are new HAL drivers and, for Phase 3, a new P4 data-plane program. The OS, AVM, and all application code remain unchanged.

---

*AERO Systems Stack Blueprint v1.0*
