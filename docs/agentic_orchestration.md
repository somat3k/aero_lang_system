# AERO Agentic Orchestration — Linear Integration & Warehouse Graveler

**Document Type:** Feature Specification  
**Version:** 0.1  
**Status:** Draft

---

## Table of Contents

1. [Overview](#1-overview)  
2. [Goals](#2-goals)  
3. [Linear-Driven Agentic Session Flow](#3-linear-driven-agentic-session-flow)  
4. [Component Architecture](#4-component-architecture)  
5. [Rust CLI Package (`aero-orchestrator`)](#5-rust-cli-package-aero-orchestrator)  
6. [Macro & Mapping Layer (IDE + Devices)](#6-macro--mapping-layer-ide--devices)  
7. [Runtime Pylons, Shards, and Clusters](#7-runtime-pylons-shards-and-clusters)  
8. [Warehouse Graveler Data Hub](#8-warehouse-graveler-data-hub)  
9. [APIs & Protocol Surfaces](#9-apis--protocol-surfaces)  
10. [Observability, Policy, and Enforcement](#10-observability-policy-and-enforcement)  
11. [Incremental Rollout](#11-incremental-rollout)

---

## 1. Overview

This specification defines how AERO links **Linear projects** to **orchestrated agentic sessions**. A Linear mention (`@agent <content>`) becomes a typed request that spins up a skill-scoped session, executes enforced instructions, and reports progress back to Linear. The experience is delivered as a **standalone Rust package with a CLI** and exposed via gRPC/Proto for IDEs, devices, and other runtimes.

The system also introduces **Warehouse Graveler** — a data transformer and interstellar hub that turns heterogeneous inputs into a graph of typed queries, backed by gRPC and a Graph Neural Network (GNN) for advanced classification and routing.

---

## 2. Goals

| # | Goal |
|---|------|
| G1 | Zero-friction request path from Linear: an `@agent` mention creates a session with explicit instructions and capability bounds. |
| G2 | First-class Rust package + CLI so teams can run orchestration locally, in CI, or as a managed service. |
| G3 | Macro/mapping layer that works across languages and IDEs (proc-macros, decorators, code lenses) without duplicating logic. |
| G4 | Runtime awareness of shards/clusters with metrics, skill inventory, and automatic discovery of available agents. |
| G5 | Warehouse Graveler transforms and classifies any data stream, producing typed knowledge usable by agents via gRPC/Proto and GNN routing. |

---

## 3. Linear-Driven Agentic Session Flow

1. **Intent** — User comments in Linear: `@aero-agent classify dataset X with policy Y`.
2. **Webhook Intake** — Linear webhook posts to `aero-orchestrator` (`/linear/intake`), carrying issue context, actor, labels, and the mention payload.
3. **Instruction Binding** — Orchestrator extracts structured instructions (skills, constraints, data refs) and binds them to a **Session** object with a capability token.
4. **Shard Selection** — Scheduler picks shards (or spins new ones) matching requested skills and locality constraints; pylons route the session to the chosen shard/cluster.
5. **Execution** — Agents run the task, calling Warehouse Graveler for transforms/classification when needed.
6. **Feedback Loop** — Progress, metrics, and artifacts stream back to Linear (comments, status fields) and to observability sinks.
7. **Closure** — Session ends with a signed report; capabilities revoked; deltas persisted to the knowledge surface.

---

## 4. Component Architecture

```
Linear Webhook
     │
     ▼
┌──────────────────────────┐
│   Intake & Policy Guard  │  (authz, instruction parser)
└───────────┬──────────────┘
            │
            ▼
┌──────────────────────────┐
│  Session Orchestrator    │  (schedule, enforce, reconcile)
└───────────┬──────────────┘
            │
     ┌──────┴───────────┐
     ▼                  ▼
Shard Mesh (pylons)   Warehouse Graveler
 (skills, metrics)    (transform, classify, GNN routing)
     │                  │
     ▼                  ▼
 Linear updates     gRPC/Proto to IDEs/devices
```

**Key components**

- **Linear Connector** — Webhook receiver + signer validation; maps mentions to typed session intents.  
- **Session Orchestrator** — Enforces instructions, manages lifecycle, reconciliation, and status back-propagation.  
- **Skill Registry** — Declares capabilities, resource bounds, and required adapters; used by scheduler.  
- **Pylons** — Routing primitives that bind sessions to shards/clusters based on topology, latency, and data gravity.  
- **Warehouse Graveler** — Data transformer hub with GNN-based query routing and gRPC/Proto surface.  
- **Observability/Policy Plane** — Metrics, traces, audit logs, and policy-as-code for instructions.

---

## 5. Rust CLI Package (`aero-orchestrator`)

CLI delivered as an independent Rust crate to keep orchestration portable and auditable.

**Command surface (initial)**

```bash
aero-orch init linear --api-key $LINEAR_TOKEN --team-id T123
aero-orch register-skill classify --image docker.io/aero/classifier:edge --caps data.read,ml.infer
aero-orch start-session --from-linear ISS-42 --skill classify --instruction "label @sensitive"
aero-orch attach-instruction --session S123 --file policy.yaml
aero-orch sync --push-status --emit-metrics
```

**Behaviors**
- Offline-first: works with local state; syncs when connected.  
- Policy-aware: refuses instructions that exceed declared capabilities.  
- Extensible transports: HTTP/gRPC for service mode; stdio for editor/IDE plugins.  
- Ship as static binaries + crates.io dependency; optional WASM target for IDE embeddings.

---

## 6. Macro & Mapping Layer (IDE + Devices)

- **Rust proc-macros** for declaring skills and instruction schemas: `#[skill(name = "classify", caps = ["data.read"])]`.
- **Language bridge**: TS/JS decorators and Python annotations generate the same session/skill schema via codegen (no divergent logic).
- **IDE affordances**: code lens and quick-fix templates to emit `@agent` payloads directly from editors; uses the CLI in stdio mode.
- **Device profiles**: mappings for constrained devices (edge/IoT) limit skill sets and enforce sandboxing.
- **Mutant-based prototyping**: hot-reload macros so skill definitions can be mutated safely in dev; reconciler ensures schema drift is detected before deployment.

---

## 7. Runtime Pylons, Shards, and Clusters

- **Shards**: execution islands with defined skill inventory and resource budgets.  
- **Clusters**: collections of shards with shared telemetry and placement rules.  
- **Pylons**: routing waypoints that steer sessions toward shards based on latency, data locality, or regulatory domain.  
- **Auto-discovery**: shards publish heartbeats and skill manifests; orchestrator updates placement decisions dynamically.  
- **Metrics**: queue depth, execution latency, policy violations, skill saturation, and Linear round-trip time.

---

## 8. Warehouse Graveler Data Hub

The Graveler is a transformer for **all data types**:

- **Ingress**: file drops, object storage events, streams, DB change feeds, and sensor inputs.  
- **Transform**: schema inference, normalization, embedding, and classification pipelines.  
- **Graph Neural Network of Queries**: each request becomes a node; edges capture data lineage and dependency; GNN ranks optimal execution paths and routing.  
- **Protocols**: gRPC/Proto for structured ingestion; HTTP for lightweight clients; optional Kafka adapter for high-throughput lanes.  
- **Interoperability**: adapters for network planes/lanes controllers and ledger/shard coordinators so data and control traffic share the same typed graph.  
- **Outputs**: typed knowledge surfaces consumable by agents; redaction and sensitivity tags propagated.

Performance targets (initial): ingest 10k events/s per node, P95 transform latency < 200 ms for standard classification pipelines.

---

## 9. APIs & Protocol Surfaces

**gRPC/Proto (sketch)**

```proto
service SessionService {
  rpc IntakeLinear(LinearEvent) returns (Session);
  rpc StartSession(SessionSpec) returns (Session);
  rpc AppendInstruction(Instruction) returns (Session);
  rpc StreamStatus(SessionRef) returns (stream SessionStatus);
}

service GravelerService {
  rpc Ingest(DataChunk) returns (IngestAck);
  rpc Classify(ClassifyRequest) returns (ClassifyResult);
  rpc QueryGraph(QueryGraphRequest) returns (QueryGraphResult);
}
```

**Data model (core)**

- `Session`: id, origin (Linear), skills, capability token, pylons path, shard binding, status.  
- `Instruction`: content, enforcement policy, attachments, expiry.  
- `Skill`: name, version, required adapters, resource limits, device profile.  
- `Shard`: id, cluster, skills, load metrics, locality hints.

---

## 10. Observability, Policy, and Enforcement

- **Policies**: instruction allow/deny, data residency, sensitivity tags, max runtime, retry/backoff.  
- **Audit**: signed session reports, Linear comment thread linkage, immutable policy decisions.  
- **Metrics**: per-session SLA, GNN routing confidence, Graveler transform latency, webhook success rate.  
- **Traces**: end-to-end spans from Linear intake → shard execution → Graveler transform → Linear update.  
- **Safety**: capability tokens bound to sessions; no implicit privilege escalation when macros mutate skills.

---

## 11. Incremental Rollout

1. **Phase A — Intake & CLI Skeleton**: Linear webhook intake, session model, `aero-orch` CLI with `init`, `start-session`, `sync`.  
2. **Phase B — Shards & Pylons**: skill registry, placement engine, metrics export, IDE stdio integration.  
3. **Phase C — Macros Across Environments**: proc-macros + TS/Python codegen, device profiles, mutation-safe hot reload.  
4. **Phase D — Warehouse Graveler**: ingestion, transform/classify, GNN routing, gRPC/Proto GA.  
5. **Phase E — Hardening**: policy enforcement, audit, SLO dashboards, scale/perf tuning.
