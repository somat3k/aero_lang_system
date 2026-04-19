# AeroOS — Programming-Oriented Operating System Design

**Document Type:** OS Design Specification  
**Version:** 1.0  
**Companion:** [Systems Stack Blueprint](./systems_stack.md)

---

## Overview

AeroOS is the operating system layer of the AERO systems stack. It is not a general-purpose OS — it is a **programming-oriented OS** built specifically to host the AERO Virtual Machine and to enforce the same design contract (capability-based, observable, resilient, declarative) that the AERO language enforces at the application level.

The central thesis of AeroOS is: **a modern OS should not be a bag of POSIX compatibility shims bolted onto a 1970s process model. It should be a typed, capability-controlled, observable substrate that makes running correct programs its primary job.**

AeroOS is inspired by microkernels (seL4, Fuchsia), unikernels (MirageOS), and Erlang's "let it crash" philosophy — but is implemented in AERO and C (for the small bootloader and hardware-initialisation shim).

---

## Table of Contents

1. [Design Goals](#1-design-goals)
2. [Architecture Overview](#2-architecture-overview)
3. [Kernel Design](#3-kernel-design)
4. [Process and Address Space Model](#4-process-and-address-space-model)
5. [Capability System](#5-capability-system)
6. [Memory Management](#6-memory-management)
7. [Scheduler](#7-scheduler)
8. [Inter-Process Communication (IPC)](#8-inter-process-communication-ipc)
9. [Device Drivers](#9-device-drivers)
10. [Filesystem](#10-filesystem)
11. [Live-Reload and Hot-Swap](#11-live-reload-and-hot-swap)
12. [Security Model](#12-security-model)
13. [Observability](#13-observability)
14. [Boot Sequence](#14-boot-sequence)

---

## 1. Design Goals

| # | Goal |
|---|------|
| G1 | **Host the AVM efficiently** — the OS is optimised for green-thread workloads, not for running thousands of separate heavyweight processes |
| G2 | **Capability-based access control at every syscall** — no ambient authority, no POSIX permission strings |
| G3 | **Small, auditable TCB** — the kernel is ≤ 50 000 lines; every line is reviewed; the core is formally verified |
| G4 | **Observable by default** — the kernel emits structured telemetry compatible with the AERO Observability Pipeline |
| G5 | **Live-reload** — module hot-swap without reboot, from the bootloader to user programs |
| G6 | **No POSIX baggage** — no `/proc`, no fork/exec, no signal handling, no arbitrary file descriptors |
| G7 | **Multi-tenant isolation** — hardware-isolated sandboxes via IOMMU for untrusted workloads |

---

## 2. Architecture Overview

```
User Space
──────────────────────────────────────────────────────────────────
│  AERO Programs   │  AVM Runtime   │  OS Services (daemons)    │
│  (capability-    │  (trusted;     │  (storage, network mgmt,  │
│   isolated)      │  single addr.) │   config; typed IPC)      │
──────────────────────────────────────────────────────────────────
Kernel Space
──────────────────────────────────────────────────────────────────
│  Capability Table  │  Scheduler  │  IPC Broker  │  Memory Mgr │
│  (per-process      │  (real-time │  (typed msg  │  (IOMMU +   │
│   capability map)  │   + normal) │   passing)   │   virtual)  │
──────────────────────────────────────────────────────────────────
Hardware Abstraction Layer (HAL)
──────────────────────────────────────────────────────────────────
│  CPU init  │  IOMMU  │  NIC driver │  NVMe driver │  TPM 2.0  │
──────────────────────────────────────────────────────────────────
Hardware (x86-64 / ARM64)
```

The kernel contains **only** what cannot be done safely in user space:
- Capability table management,
- Address space switching,
- Interrupt routing,
- IPC fast-path,
- Physical memory allocation.

Everything else (drivers, filesystems, network stacks) runs in user space as **privileged OS services** that hold hardware capability tokens.

---

## 3. Kernel Design

### 3.1 Monolithic vs. Microkernel

AeroOS uses a **hybrid design**:
- The **inner kernel** is microkernel-sized (~10 000 lines), handling only capability management, address spaces, and IPC.
- The **outer kernel** includes trusted OS services (drivers, scheduler, memory manager) running with kernel-mode capability tokens but in logically separate modules, hot-swappable independently.

This gives microkernel isolation benefits without microkernel IPC overhead for trusted services.

### 3.2 Kernel Interface (syscall table)

AeroOS exposes **28 syscalls** (compared to Linux's ~400). Every syscall takes a capability token as its first argument:

| Syscall | Arguments | Description |
|---------|-----------|-------------|
| `cap_create` | `(parent_cap, rights_mask)` | Derive a child capability |
| `cap_revoke` | `(cap)` | Revoke a capability and all its children |
| `cap_inspect` | `(cap)` | Read a capability's rights and type |
| `mem_alloc` | `(mem_cap, size, flags)` | Allocate physical pages |
| `mem_map` | `(mem_cap, vaddr, paddr, flags)` | Map pages into address space |
| `mem_protect` | `(mem_cap, vaddr, size, flags)` | Change page protection flags |
| `mem_free` | `(mem_cap, paddr, size)` | Release physical pages |
| `proc_create` | `(proc_cap, image, caps[])` | Create a new process with a set of capabilities |
| `proc_exit` | `(exit_code)` | Terminate current process |
| `proc_wait` | `(proc_cap)` | Wait for a child process to exit |
| `chan_create` | `(proc_cap, type)` | Create a typed IPC channel |
| `chan_close` | `(chan_cap)` | Close and invalidate an IPC channel |
| `ipc_send` | `(chan_cap, msg_ptr, msg_len)` | Send typed IPC message |
| `ipc_recv` | `(chan_cap, buf_ptr, buf_len)` | Receive IPC message (blocking) |
| `ipc_call` | `(chan_cap, msg_ptr, msg_len, reply_buf)` | Send + receive (synchronous RPC) |
| `irq_claim` | `(irq_cap, irq_num)` | Claim an interrupt line |
| `irq_wait` | `(irq_cap)` | Block until interrupt fires |
| `dev_claim` | `(dev_cap, device_id)` | Claim ownership of a hardware device |
| `sched_yield` | `()` | Voluntarily yield the CPU |
| `sched_affinity` | `(proc_cap, cpu_mask)` | Pin a process to a CPU set |
| `futex_wait` | `(mem_cap, addr, expected)` | Block until futex value changes |
| `futex_wake` | `(mem_cap, addr, count)` | Wake blocked futex waiters |
| `timer_create` | `(clock_cap, deadline, flags)` | Create a one-shot or periodic timer |
| `timer_wait` | `(timer_cap)` | Block until timer fires |
| `clock_get` | `(clock_cap)` | Read monotonic / wall clock |
| `entropy` | `(entropy_cap, buf_ptr, buf_len)` | Read cryptographically secure random bytes |
| `debug_log` | `(log_cap, msg_ptr, msg_len)` | Emit kernel log event |
| `attest` | `(tpm_cap, nonce)` | Request TPM attestation quote |

All other functionality is implemented in user-space OS services.

---

## 4. Process and Address Space Model

### 4.1 Process Types

| Type | Description | Address Space | Isolation |
|------|-------------|---------------|-----------|
| **Kernel process** | Trusted OS service (drivers, memory manager) | Shared kernel space | Capability-gated |
| **AVM host process** | The AERO VM process | Private address space | Hardware page tables |
| **Sandbox process** | Untrusted or multi-tenant workload | Private, IOMMU-isolated | Hardware + IOMMU |

### 4.2 Single-Address-Space for AVM

The AVM runs in a **single address space** shared across all AERO actors it hosts. Actor isolation is enforced by the AVM's type system and capability model — not by separate page tables. This eliminates TLB shootdown and context-switch overhead for intra-AVM communication, enabling millions of actors to communicate at sub-microsecond latencies.

For multi-tenant scenarios where AERO programs from different trust domains must coexist on the same node, each trust domain gets its own AVM process with full hardware page-table isolation.

### 4.3 No Fork

AeroOS has no `fork` syscall. New processes are created with `proc_create`, which takes an explicit image and an explicit capability set. There is no copy-on-write inherited state. This eliminates a class of security vulnerabilities (inherited file descriptors, capability leaks across fork) and makes process creation semantics deterministic.

---

## 5. Capability System

### 5.1 Capability Types

Every resource in AeroOS is named by a **capability token** — an unforgeable, typed reference:

| Capability Type | Grants Access To |
|-----------------|-----------------|
| `MemoryCap` | Physical memory region |
| `ProcessCap` | A process (create, kill, inspect) |
| `IpcChannelCap` | An IPC channel endpoint |
| `IrqCap` | A hardware interrupt line |
| `NicCap` | A network interface (or VF via SR-IOV) |
| `StorageCap` | A storage device or partition |
| `ClockCap` | System clock |
| `TpmCap` | TPM operations |
| `LogCap` | Kernel log emission |

### 5.2 Capability Derivation

Capabilities form a tree. A process can only derive a child capability with a subset of its own rights. It cannot manufacture capabilities or escalate rights:

```
Root capability (boot time, held by kernel)
  └── MemoryCap(all) → splits into MemoryCap(region A), MemoryCap(region B)
       └── MemoryCap(region A, read-only) → granted to untrusted process
```

### 5.3 Capability Revocation

When a capability is revoked, **all descendants are automatically revoked**. This makes decommissioning a service, unmounting a filesystem, or revoking network access to a compromised process an O(1) kernel operation.

---

## 6. Memory Management

### 6.1 Physical Memory

Physical memory is managed by the kernel **Physical Memory Manager (PMM)**, which tracks pages as typed capability regions. Pages can be in the following states:

```
Free → Allocated → Mapped (in address space) → Shared (IPC) → Freed
```

### 6.2 Virtual Memory

Each process has a **virtual address space** managed by the **Virtual Memory Manager (VMM)**, which uses IOMMU-backed page tables. The kernel does not implement demand paging for AVM host processes (memory is committed at allocation time for predictable latency); demand paging is available for sandbox processes.

### 6.3 Memory Isolation

| Technique | Used For |
|-----------|----------|
| Page table separation | Per-process isolation |
| IOMMU mapping | Device DMA isolation (prevents rogue DMA attacks) |
| Memory encryption (AMD SME / Intel TME) | Confidential computing workloads |
| Guard pages | Stack overflow detection |

---

## 7. Scheduler

AeroOS implements a **two-level scheduler**:

**Level 1 — OS Scheduler (kernel)**  
A real-time-capable, priority-based scheduler for OS service threads and the AVM host process. Uses a weighted fair-queue algorithm with explicit priority bands:

| Band | Priority | Used By |
|------|----------|---------|
| 0 | Highest | Interrupt handlers, TPM, time-critical drivers |
| 1 | High | AVM host process(es) |
| 2 | Normal | OS services (network, storage) |
| 3 | Low | Background tasks, telemetry export |

**Level 2 — AVM Scheduler (user space)**  
The AVM's own work-stealing green-thread scheduler (described in [architecture.md](./architecture.md)). The OS scheduler sees the AVM as a single thread group; the AVM scheduler handles multiplexing internally.

This two-level design means **AERO programs never compete with kernel code for CPU time** — the OS scheduler guarantees the AVM host process always gets its allocated quantum.

---

## 8. Inter-Process Communication (IPC)

### 8.1 Typed IPC Channels

IPC in AeroOS uses **typed message channels** — the same typed message-passing model as AERO actors. Messages are strongly typed AERO structs; the kernel validates the message shape at the IPC boundary.

```aero
// Declare an IPC service interface
ipc interface StorageService {
    Read  { lba: u64, len: u32 } -> Result<Vec<u8>, IoError>
    Write { lba: u64, data: Vec<u8> } -> Result<(), IoError>
    Flush {} -> Result<(), IoError>
}
```

### 8.2 IPC Mechanisms

| Mechanism | Latency | Use Case |
|-----------|---------|---------|
| **Synchronous call** (`ipc_call`) | ~500 ns | Request-response to OS services |
| **Async send** (`ipc_send`) | ~100 ns | Fire-and-forget, event notification |
| **Shared memory** (mapped via `MemoryCap`) | ~10 ns | High-throughput data transfer |

### 8.3 Zero-Copy IPC

For large payloads, AeroOS supports **zero-copy IPC**: the sender grants the receiver a `MemoryCap` pointing to the payload region. The receiver maps it into its own address space and reads directly from the sender's memory, without copying. The `MemoryCap` grant is atomic — either it succeeds and the memory is accessible, or it fails and nothing is transferred.

---

## 9. Device Drivers

### 9.1 User-Space Drivers

All device drivers in AeroOS run in **user space** as privileged OS service processes. They hold `IrqCap`, `MemoryCap` (for MMIO regions), and `NicCap` / `StorageCap` tokens as appropriate.

Benefits:
- A buggy driver cannot corrupt kernel memory (no kernel-mode driver code),
- Drivers are hot-swappable without rebooting,
- Drivers are written in AERO and benefit from the type system, effect system, and AVM observability.

### 9.2 Kernel-Bypass Networking

For the network data plane, AeroOS supports **kernel-bypass** via:

| Technology | Description |
|-----------|-------------|
| **XDP (eBPF)** | In-kernel fast-path for packet filtering/forwarding before the driver stack |
| **DPDK** | Full user-space NIC driver, zero-copy packet I/O at line rate |
| **SR-IOV Virtual Functions** | Hardware-isolated NIC slices for multi-tenant workloads |

The AERO network control plane (described in [network_architecture.md](./network_architecture.md)) programs the data plane via a capability-gated API.

---

## 10. Filesystem

### 10.1 Design

AeroOS uses a **log-structured, capability-gated filesystem** called **AeroFS**:

- All file/directory access requires a `StorageCap` with path-scoped rights,
- Files are identified by UUIDs, not by path strings (paths are a naming layer on top),
- Writes are always append-only in the log; garbage collection compacts old revisions,
- Every file has an integrity checksum (SHA-256 by default); silent corruption is detected on every read,
- Snapshots and rollback are O(1) operations (just a log position pointer).

### 10.2 No `/proc`, No `/dev`

AeroOS has no `/proc` pseudo-filesystem. Process introspection is done via typed IPC to a process-management OS service that holds `ProcessCap` tokens. Devices are accessed via capability tokens, not via device file paths.

---

## 11. Live-Reload and Hot-Swap

AeroOS supports **live-reload at every layer**:

| Layer | Mechanism |
|-------|-----------|
| User program | AVM hot-code replacement (see [architecture.md](./architecture.md)) |
| AVM runtime | Load new AVM binary alongside old; migrate processes; unload old |
| OS service (driver) | Stop → verify new image → start new → revoke old capabilities |
| Kernel module | Outer kernel modules only; inner kernel requires reboot |

The OS service lifecycle is managed by a **Service Supervisor** (an OS service that itself runs under the kernel's supervision):

```
ServiceSupervisor
  ├── NicDriver        (restartable, hot-swappable)
  ├── StorageDriver    (restartable, hot-swappable)
  ├── NetworkDaemon    (restartable, hot-swappable)
  ├── AeroFS           (restartable with state handoff)
  └── AVM              (hot-reload via module replacement)
```

This mirrors the AERO language supervision tree model — the same concepts apply at the OS level.

---

## 12. Security Model

### 12.1 Secure Boot and Measured Boot

See [systems_stack.md §Hardware](./systems_stack.md) for the full secure boot chain. The OS kernel verifies the bootloader signature and extends TPM PCRs at each boot stage.

### 12.2 No Ambient Authority

At no point in AeroOS can a process obtain a capability it was not explicitly granted. There are no SETUID binaries, no ambient network access, no ambient filesystem access. A process that was not given a `NicCap` cannot send a single byte on the network — even if it runs as "root".

### 12.3 Privilege Separation

The kernel itself is split into:
- **Inner kernel** — formally verified, minimal, handles only capability management and page tables.
- **Outer kernel** — trusted but not formally verified; holds capability tokens; hot-swappable.
- **OS services** — user space, capability-gated; each has the minimum capabilities needed.

### 12.4 Side-Channel Mitigations

| Threat | Mitigation |
|--------|-----------|
| Spectre / Meltdown | Kernel page-table isolation (KPTI) enabled by default |
| Rowhammer | ECC RAM required; TRR-aware memory allocator |
| Cache timing attacks | Constant-time cryptographic primitives; cache partitioning for sensitive workloads |
| DMA attacks | IOMMU active for all devices; no device can DMA outside its allocated region |

---

## 13. Observability

AeroOS emits structured telemetry from every kernel subsystem using the same event schema as the AERO Observability Pipeline:

| Subsystem | Events Emitted |
|-----------|---------------|
| Scheduler | context-switch, quantum-expire, process-create, process-exit |
| Memory | alloc, free, page-fault, oom-kill |
| IPC | send, recv, call-latency, timeout |
| Drivers | irq-received, dma-complete, error |
| Security | cap-denied, cap-revoked, attestation-request |

All events are emitted to a **kernel telemetry ring buffer** that the AVM telemetry daemon drains and exports via OTLP. This means OS-level events appear in the same trace and metric streams as application-level events — there is no separate "kernel monitoring" system.

---

## 14. Boot Sequence

```
Power On
  │
  ▼
UEFI firmware (measured, Secure Boot enabled)
  │  verifies
  ▼
AeroOS bootloader (256 KB, AERO + assembly)
  │  loads and verifies
  ▼
AeroOS inner kernel
  │  initialises:
  │  • CPU (SMP, interrupt routing, IOMMU)
  │  • PMM (physical memory map from UEFI)
  │  • Capability root
  │  • IPC broker
  │  then launches:
  ▼
Service Supervisor (first user-space process)
  │  launches in order:
  ▼
NicDriver → StorageDriver → AeroFS → NetworkDaemon → AVM
  │
  ▼
AVM loads AERO Standard Library + user programs
  │
  ▼
System Ready  (target: < 500 ms from power-on to first request served)
```

---

*AeroOS Design Specification v1.0*
