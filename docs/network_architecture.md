# AERO Network Architecture — Lean, Programmable Network Layer

**Document Type:** Network Architecture Specification  
**Version:** 1.0  
**Companion:** [Systems Stack Blueprint](./systems_stack.md)

---

## Overview

The AERO network layer is designed on a single principle: **the network is programmable infrastructure, not a fixed black box.** Every forwarding decision, policy rule, and topology change is an explicit, typed, version-controlled artifact — not a manual CLI command entered into a switch.

This document specifies how to build such a network from commodity hardware (Phase 1) and how it evolves toward custom programmable NICs and smart switches (Phase 2+).

---

## Table of Contents

1. [Design Goals](#1-design-goals)
2. [Planes Model](#2-planes-model)
3. [Data Plane](#3-data-plane)
4. [Control Plane](#4-control-plane)
5. [Management Plane](#5-management-plane)
6. [Physical Topology](#6-physical-topology)
7. [Addressing and Routing](#7-addressing-and-routing)
8. [Security](#8-security)
9. [Observability](#9-observability)
10. [Evolution Path (Phase 2+)](#10-evolution-path-phase-2)

---

## 1. Design Goals

| # | Goal |
|---|------|
| N1 | **Programmable at every layer** — forwarding rules, routing policy, and topology are code, not CLI configuration |
| N2 | **Declared state** — the desired network state is expressed declaratively; the control plane reconciles actual state toward it |
| N3 | **Failure as first-class input** — link failures, congestion, and mis-configuration are typed events, not silent degradation |
| N4 | **Observable by default** — every packet path, flow, and forwarding decision is traceable |
| N5 | **Zero-trust by default** — no implicit trust between any two endpoints; all communication is authenticated and authorised |
| N6 | **Lean operational model** — the entire network for a production cluster can be managed by a single engineer without specialised network hardware expertise |
| N7 | **Kernel-bypass performance** — the data plane achieves line-rate forwarding without touching the kernel network stack |

---

## 2. Planes Model

Every network function is explicitly assigned to one of three planes. No function spans planes without a typed API boundary:

```
┌─────────────────────────────────────────────────────────────────┐
│  Management Plane                                                │
│  What state should the network be in?                           │
│  AERO world-type bindings, gRPC API, GitOps declarative config  │
├─────────────────────────────────────────────────────────────────┤
│  Control Plane                                                   │
│  How does the network get to the desired state?                  │
│  AERO actors, BGP/OSPF agents, topology reconciliation daemon   │
├─────────────────────────────────────────────────────────────────┤
│  Data Plane                                                      │
│  Forward packets at line rate.                                   │
│  eBPF/XDP (Phase 1) │ P4 programs (Phase 2 custom NICs)        │
└─────────────────────────────────────────────────────────────────┘
```

The **management plane** expresses intent. The **control plane** translates intent into forwarding rules. The **data plane** executes those rules at hardware speed.

---

## 3. Data Plane

### 3.1 Phase 1 — eBPF/XDP (Commodity NICs)

In Phase 1, the data plane is implemented using **eBPF programs attached to the XDP hook** — the earliest possible point in the Linux network stack, before any kernel processing:

```
NIC hardware receives packet
  │
  ▼ (XDP hook — runs eBPF program)
eBPF data-plane program
  │
  ├── XDP_DROP      — discard (firewall, DDoS)
  ├── XDP_TX        — hairpin forward back out the same NIC
  ├── XDP_REDIRECT  — forward to another NIC / CPU queue
  └── XDP_PASS      — hand off to kernel network stack
```

eBPF programs are compiled from a high-level **AERO Data Plane DSL** (a subset of AERO) and loaded into the kernel via capability-gated syscalls:

```aero
// AERO Data Plane DSL example — simple L3 forwarder
#[xdp]
fn forward(ctx: XdpContext) -> XdpAction {
    know eth = ctx.parse(EtherHeader)?;
    know ip  = ctx.parse(Ipv4Header)?;

    know next_hop = fib_lookup(ip.dst_addr)?;

    ctx.rewrite_eth_dst(next_hop.mac);
    ctx.rewrite_eth_src(local_mac());
    ctx.decrement_ttl();

    XdpAction::Redirect(next_hop.ifindex)
}
```

### 3.2 Phase 1 Performance Targets

| Metric | Target | Basis |
|--------|--------|-------|
| Forwarding throughput | 40 Gbps on a single 4-core server | Intel E810, XDP benchmark |
| Forwarding latency P50 | < 5 µs | Kernel-bypass measurements |
| Forwarding latency P99 | < 20 µs | With CPU pinning and NUMA awareness |
| Connections per second | > 1 M/s | SYN-cookie offload in eBPF |

### 3.3 Phase 2 — P4 on Custom NICs

In Phase 2, eBPF is replaced with **P4 programs** running on custom or off-the-shelf programmable NICs (e.g., Pensando, Intel IPU, AMD Xilinx Alveo):

- P4 programs run directly on the NIC ASIC or FPGA — no CPU cycles at all for forwarding,
- The **AERO P4 compiler** (an extension of `aeroc`) translates AERO Data Plane DSL to P4 and then to the target NIC's native binary,
- The same AERO source file targets both XDP (Phase 1) and P4 (Phase 2) via a compile-time target flag.

```bash
aeroc build-dp --target xdp    # Phase 1: compile to eBPF/XDP
aeroc build-dp --target p4-nic # Phase 2: compile to P4 for target NIC
```

---

## 4. Control Plane

### 4.1 Control Plane as AERO Actors

The control plane is a set of **AERO actor services** running on AeroOS. Each service is responsible for one routing or policy protocol:

| Actor Service | Responsibility |
|--------------|---------------|
| `TopologyActor` | Discovers and maintains the physical/logical network graph |
| `RoutingActor` | Runs BGP (external) and OSPF/IS-IS (internal); computes FIB |
| `PolicyActor` | Enforces routing policy, traffic engineering rules, QoS |
| `ReconcilerActor` | Compares desired state (from Management Plane) with actual FIB/ACLs; pushes deltas to Data Plane |
| `FaultActor` | Monitors link state, detects failures, triggers rerouting |
| `TelemetryActor` | Collects IPFIX/sFlow/eBPF events; feeds the Observability Pipeline |

### 4.2 Desired-State Reconciliation

The control plane follows the same reconciliation pattern as the AERO World-Model subsystem:

```
Management Plane emits desired NetworkState
         │
         ▼
ReconcilerActor.observe(actual_state)  ← polls FIB, ACLs, link states
         │
ReconcilerActor.diff(desired, actual)
         │
         ├── No delta → do nothing
         │
         └── Delta exists → push to Data Plane via capability-gated API
                             emit telemetry event network.reconciliation.delta
```

This model means **the network is always self-healing**: if a manual change is made to a router outside the control plane, the ReconcilerActor will detect the drift and push the correct state back.

### 4.3 Convergence Targets

| Event | Target Convergence Time |
|-------|------------------------|
| Link failure detected | < 50 ms |
| FIB updated after link failure | < 200 ms |
| BGP route withdrawn and replaced | < 500 ms |
| Manual config drift corrected | < 1 s (next reconciliation cycle) |

---

## 5. Management Plane

### 5.1 Declarative Network Configuration

Network state is expressed as AERO world types stored in a Git repository (GitOps model). The desired network state is a set of typed values:

```aero
world NetworkTopology {
    nodes:  HashMap<NodeId, NodeConfig>,
    links:  Vec<LinkConfig>,
    routes: Vec<StaticRoute>,
    acls:   Vec<AclRule>,
    qos:    QosPolicy,
}
```

A commit to the network config repository triggers the Management Plane to compute a diff and push it to the Control Plane. Every network change is reviewed, versioned, and rollback-able with `git revert`.

### 5.2 Management API

The Management Plane exposes a **typed gRPC API** for programmatic access:

```protobuf
service NetworkManager {
    rpc GetTopology   (Empty)          returns (NetworkTopology);
    rpc ApplyConfig   (NetworkState)   returns (ApplyResult);
    rpc GetFib        (NodeId)         returns (ForwardingTable);
    rpc GetFlowStats  (FlowQuery)      returns (stream FlowRecord);
    rpc Rollback      (CommitHash)     returns (RollbackResult);
}
```

### 5.3 Day-2 Operations

All day-2 operations (firmware upgrades, capacity changes, traffic engineering adjustments) are modelled as **world-type delta emissions** — the same pattern as any other AERO program. This means the full AERO testing and simulation infrastructure applies to network operations.

---

## 6. Physical Topology

### 6.1 Recommended Spine-Leaf Architecture

For a multi-server deployment, the recommended physical topology is **spine-leaf**:

```
Spine Layer (2+ spine switches for redundancy)
─────┬─────────────────────────────────────┬─────
     │ (uplinks, ECMP)                     │
─────▼───────────────────────────────────────────
Leaf Layer (one leaf switch per rack)
  ├── Rack A (servers A1–A16) ── Leaf A
  ├── Rack B (servers B1–B16) ── Leaf B
  └── Rack C (servers C1–C16) ── Leaf C
```

- Each server has **two NIC ports** connected to two different leaf switches (active-active bonding for fault tolerance).
- Each leaf switch has **two uplinks** to two different spine switches (ECMP for load distribution).
- This provides **no single point of failure** at either the NIC, switch, or uplink level.

### 6.2 Minimum Viable Cluster

For development and early production:

```
2× commodity servers
  └── Each with 2× 25 GbE NIC ports
2× commodity switches (e.g., Wedge100, EdgeCore)
  └── Running AeroOS + AERO control plane (not a closed-source NOS)
```

Total hardware cost: ~$10 000 — entirely commodity, no vendor lock-in.

### 6.3 Storage Network

For nodes that require high-throughput storage access, a **dedicated storage fabric** is recommended:

- NVMe-oF (NVMe over Fabrics) over RDMA (RoCEv2) for remote block storage,
- Separate 100 GbE storage NICs (SR-IOV, one VF per workload),
- AeroFS (see [os_design.md](./os_design.md)) exposes storage via typed IPC; the NVMe-oF driver is a standard AERO world adapter.

---

## 7. Addressing and Routing

### 7.1 Address Space

| Layer | Scheme | Notes |
|-------|--------|-------|
| Physical (L2) | 48-bit MAC | Assigned by NIC vendor; not routed across racks |
| Underlay (L3) | RFC 1918 IPv4 or ULA IPv6 | Loopback-per-node for BGP; point-to-point /31s for links |
| Overlay (services) | IPv6 ULA /48 prefix | One /64 per logical service network |
| AERO actors (cluster) | 128-bit Actor UUID | Not an IP address; addressed via AERO Cluster Protocol |

### 7.2 Routing Protocol Stack

| Use Case | Protocol |
|----------|---------|
| Underlay (within cluster) | eBGP unnumbered (RFC 5549 IPv4 via IPv6 next-hop; RFC 7938 for BGP in data-centres) — simple, no IGP needed |
| External connectivity | eBGP with full-table or default route from upstream |
| Service discovery | AERO Service Registry (etcd-compatible) |
| DNS | CoreDNS, configured as a world adapter |

### 7.3 Why eBGP Unnumbered

eBGP "unnumbered" uses IPv6 link-local addresses for BGP peer discovery (RFC 7938, §3) and RFC 5549 to advertise IPv4 prefixes with an IPv6 next-hop — eliminating the need to manually assign IPv4 addresses to every point-to-point link:

- No need for a separate IGP (OSPF/IS-IS) — one protocol for everything,
- Each link is a BGP session; no manual IPv4 assignment needed for links (IPv6 link-local auto-config),
- Scales from 2 nodes to 10 000 nodes with no architectural change,
- Widely supported in open-source (FRRouting) and commodity switch ASICs.

---

## 8. Security

### 8.1 Zero-Trust Network Model

AERO adopts a **zero-trust network model**: no implicit trust based on IP address or network segment. Every connection is:

1. **Authenticated** — mutual TLS (mTLS) with certificates issued by the AERO Certificate Authority,
2. **Authorised** — the AERO Capability System grants specific network capabilities to specific services,
3. **Encrypted** — TLS 1.3 minimum for all service-to-service traffic; WireGuard for underlay encryption.

```
Service A wants to call Service B
  │
  ▼
Service A presents its mTLS certificate
  │
  ▼
Policy engine checks: does Service A have NicCap(target=ServiceB)?
  │
  ├── Yes → TLS handshake completes, connection allowed
  └── No  → connection rejected, audit event emitted
```

### 8.2 Network Segmentation

| Segment | Capability Required | Contents |
|---------|-------------------|---------|
| Service mesh | `NetworkCap(service)` | Inter-service communication |
| Storage fabric | `NetworkCap(storage)` | NVMe-oF traffic |
| Management | `NetworkCap(mgmt)` | Control plane, SSH, metrics |
| External | `NetworkCap(external)` | Internet-facing traffic |

Services cannot cross segment boundaries without explicit capability grants.

### 8.3 DDoS Mitigation

The eBPF/XDP data plane includes a built-in **DDoS mitigation module**:
- SYN-cookie offload (handles SYN floods in the NIC, never reaches the kernel),
- Per-source-IP rate limiting (configurable via Management Plane world-type config),
- Automatic blackhole routing for sources exceeding threshold (announced via BGP RTBH).

---

## 9. Observability

The AERO network layer emits the following telemetry into the AERO Observability Pipeline:

### 9.1 Flow-Level Metrics

Every active flow is sampled (configurable, default 1:1000) and reported as an **IPFIX flow record** enriched with AERO service identity:

| Field | Description |
|-------|-------------|
| `src_ip`, `dst_ip` | Endpoint addresses |
| `src_service`, `dst_service` | AERO service identity (from mTLS cert) |
| `bytes`, `packets` | Volume counters |
| `latency_p50`, `latency_p99` | Round-trip time distribution |
| `retransmits` | TCP retransmission count |
| `start_time`, `end_time` | Flow duration |

### 9.2 Control Plane Events

| Event | Description |
|-------|-------------|
| `network.link.up` / `network.link.down` | Link state change |
| `network.bgp.session_established` | BGP peer connected |
| `network.bgp.route_received` | New route learned |
| `network.reconciliation.delta` | Desired vs. actual drift corrected |
| `network.acl.denied` | Packet dropped by ACL |

### 9.3 Distributed Network Traces

Inter-service calls automatically inherit the AERO Trace Context. When a request traverses multiple services, the network path (including which NIC, which switch, which physical link) is recorded as a span attribute — giving full path visibility from application to hardware.

---

## 10. Evolution Path (Phase 2+)

| Phase | Timeline | Capability Added |
|-------|----------|-----------------|
| **Phase 1** | Now | eBPF/XDP data plane, eBGP control plane, commodity NICs and switches |
| **Phase 2** | Post-v1.0 | Custom appliance boards, P4 NICs, in-network computing, RDMA fabric |
| **Phase 3** | Long-term | Custom SmartNIC ASIC, hardware offload of AERO actor message routing |

The key design invariant is that **Phase 2 and 3 changes are isolated to the data plane and HAL**. The control plane, management plane, and all application code continue to work without modification.

---

*AERO Network Architecture Specification v1.0*
