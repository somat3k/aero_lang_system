# HoloLang — AERO Domain Bridge for Holographic & Physical Systems

**Document Type:** Domain Bridge Language Specification  
**Version:** 1.0  
**Compiles To:** AERO bytecode (AVM) via the `aeroc` compiler with the `--domain holographic` flag

> *"Operators are not written, they are pre-compiled. Context is not configured, it is discovered."*

---

## Overview

**HoloLang** is a domain-specific language that sits above AERO and compiles to AERO bytecode. It provides a high-level declarative surface for orchestrating physical device systems — lasers, mirrors, sensors, projectors, arrays — combined with a computation graph model backed by GEMM-accelerated tensor operations and a spatial canvas model for holographic projection.

HoloLang does not replace AERO. It **bridges to AERO**. Every HoloLang construct compiles to well-typed AERO actors, world types, and effect-tracked functions. The bridge is not a layer of indirection — it is a pre-compilation pass that expands domain-specific constructs into optimised AERO code before the AERO compiler sees them.

```
HoloLang source (.hl)
        │
        │  aeroc --domain holographic
        ▼
AERO IR (actors + world types + effects)
        │
        │  aeroc --emit avm
        ▼
AVM bytecode  →  GEMM-accelerated kernel  →  Device drivers
```

---

## Table of Contents

1. [Design Goals](#1-design-goals)
2. [Discovery-Dynamics-Adaptable Computation (DDAC)](#2-discovery-dynamics-adaptable-computation-ddac)
3. [Session Model](#3-session-model)
4. [Device Declarations](#4-device-declarations)
5. [SafeTensor — Bounds-Checked Beam Data](#5-safetensor--bounds-checked-beam-data)
6. [Computation Graph](#6-computation-graph)
7. [GEMM Acceleration](#7-gemm-acceleration)
8. [MDI Canvas — Spatial Mesh Model](#8-mdi-canvas--spatial-mesh-model)
9. [Communication Channels](#9-communication-channels)
10. [Enumerations and Directional Control](#10-enumerations-and-directional-control)
11. [Skill Tracking](#11-skill-tracking)
12. [DocDirectory Generation](#12-docdirectory-generation)
13. [Pooled VM Runtime](#13-pooled-vm-runtime)
14. [Complete System Example](#14-complete-system-example)
15. [Compiled AERO Output — Annotated](#15-compiled-aero-output--annotated)

---

## 1. Design Goals

| Priority | Goal | Description |
|----------|------|-------------|
| H1 | **Pre-compilation** | Functional operators and symbols are resolved at compile time, not at runtime. No dynamic dispatch on the critical projection path. |
| H2 | **Contextual projection** | Constructs are semantically aware of the projection screening condition they will execute under — the compiler validates compatibility before code is emitted. |
| H3 | **Smooth transition** | DDAC ensures the computation graph adapts gracefully when environmental conditions change — no abrupt mode switches, no restarts. |
| H4 | **GEMM acceleration** | All matrix-heavy operations (beam geometry, tensor transformations, sensor fusion) are lowered to GEMM kernels and dispatched to the appropriate accelerator (GPU, tensor core, or CPU SIMD). |
| H5 | **Device safety** | Physical device bounds (angle limits, power limits, frequency limits) are encoded in the type system. Out-of-bound commands are compile-time errors, not runtime faults. |
| H6 | **Zero silent failures** | Every device interaction, every channel message, every tensor operation is observable by default. There is no silent data path. |

---

## 2. Discovery-Dynamics-Adaptable Computation (DDAC)

DDAC is HoloLang's core computation model. It addresses a fundamental problem in physical device control: the environment changes continuously, and the computation graph that was optimal at startup may not be optimal ten seconds later.

### The Three Phases of DDAC

```
┌──────────────────────────────────────────────────────────┐
│                    DDAC Cycle                            │
│                                                          │
│  ① DISCOVER → ② DYNAMICS → ③ ADAPT                      │
│                                                          │
│  ① Discover: probe sensors, characterise environment,    │
│     build initial computational profile                  │
│                                                          │
│  ② Dynamics: continuously observe environmental drift,   │
│     track computation graph performance metrics          │
│                                                          │
│  ③ Adapt: recompile the hot path in background,         │
│     hot-swap the execution plan when ready               │
└──────────────────────────────────────────────────────────┘
```

### DDAC Annotation

```hololang
@ddac(
    probe_interval: "50ms",     // how often to sample environment
    adapt_threshold: 0.05,      // 5% performance drift triggers replan
    warmup_cycles:  3,          // stable cycles before committing new plan
)
```

### What DDAC Discovers

- **Screen geometry** — distance, angle, surface reflectivity
- **Ambient light levels** — adjusts beam power on the fly
- **Thermal drift** — recalibrates mirror galvanometer offsets as temperature changes
- **Sensor response curves** — adapts sampling strategy per device
- **Network jitter** — adjusts gRPC streaming window and batch sizes

### DDAC and the Computation Graph

When DDAC triggers an adaptation, it does not restart the session. It:
1. Builds an alternative computation graph in a shadow micro-environment,
2. Validates the new plan against the current projection condition,
3. Performs a smooth crossfade transition (overlap window configurable),
4. Promotes the new plan and retires the old one.

This is AERO's micro-environment model applied to real-time physical systems.

---

## 3. Session Model

A session is the top-level execution scope in HoloLang. It owns all devices, canvases, channels, and skills declared within it.

```hololang
@session("full-system-demo")
```

A session compiles to an AERO actor supervisor tree with:
- One root supervisor actor per session
- One device-manager actor per device group
- One canvas-manager actor per MDI canvas
- One channel-manager actor per communication channel

### Session Lifecycle

```
spawn → discover (DDAC ①) → run → [adapt loop (DDAC ②③)] → teardown
```

Sessions support **graceful teardown**: all devices are driven to their safe states (laser off, mirrors to home position) before the session actor terminates.

### `@session` Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `name` | String | required | Unique session identifier |
| `log_level` | Enum | `info` | Session-wide log verbosity |
| `telemetry` | Bool | `true` | Enable AERO telemetry pipeline |
| `safe_state_on_exit` | Bool | `true` | Drive devices to safe state on shutdown |
| `ddac` | Object | see §2 | DDAC tuning parameters |

---

## 4. Device Declarations

Devices are typed hardware descriptors. They compile to AERO `world<DeviceState>` types backed by a device-driver micro-environment.

```hololang
device GreenLaser {
    type:          "solid-state-laser"
    wavelength_nm: 532
    max_power_mw:  150
    modulation:    "cw"
}

device MirrorH {
    type:    "galvanometer-mirror"
    axis:    "horizontal"
    min_deg: -30
    max_deg:  30
}

device SensorA {
    type:           "photodiode"
    sample_rate_hz: 5000
}
```

### Type-Safe Bounds

The bounds declared in a device block (`max_power_mw`, `min_deg`/`max_deg`, etc.) are lifted into the compiled AERO type. Attempts to command a device outside its declared bounds produce a **compile-time error**.

```hololang
// ✅ Valid — within declared max_deg: 30
GreenLaser.set_angle(MirrorH, 25.0);

// ❌ Compile-time error: 35.0 exceeds MirrorH.max_deg (30.0)
GreenLaser.set_angle(MirrorH, 35.0);
```

### Device Groups

Related devices can be grouped for coordinated control:

```hololang
device_group BeamSystem {
    laser:   GreenLaser
    mirror_h: MirrorH
    mirror_v: MirrorV
}
```

The group compiles to a single AERO actor that coordinates the member devices in lock-step with configurable synchronisation guarantees.

### Supported Device Types

| Type String | Hardware Class | Key Parameters |
|-------------|----------------|----------------|
| `solid-state-laser` | CW laser source | `wavelength_nm`, `max_power_mw` |
| `diode-laser` | Pulsed laser source | `wavelength_nm`, `max_power_mw`, `pulse_width_us` |
| `galvanometer-mirror` | Galvo-mirror | `axis`, `min_deg`, `max_deg`, `max_speed_deg_s` |
| `photodiode` | Point sensor | `sample_rate_hz`, `sensitivity_mv_mw` |
| `ccd` | Area sensor | `sample_rate_hz`, `resolution_px` |
| `dmd` | Digital micromirror | `resolution_px`, `frame_rate_hz` |
| `slm` | Spatial light modulator | `resolution_px`, `bit_depth` |

---

## 5. SafeTensor — Bounds-Checked Beam Data

`SafeTensor<T, Shape>` is HoloLang's primary data container for beam geometry, sensor readings, and intermediate computation values. It is bounds-checked at the type level — the shape is part of the type, so shape mismatches are compile-time errors.

```hololang
// Declare a 512×512 single-precision beam intensity map
know beam_map: SafeTensor<f32, [512, 512]> = SafeTensor::zeros();

// Declare a 3-element position vector
know position: SafeTensor<f64, [3]> = SafeTensor::from([x, y, z]);

// Declare a 4×4 homogeneous transformation matrix
know transform: SafeTensor<f64, [4, 4]> = SafeTensor::identity();
```

### Compile-Time Shape Checking

```hololang
know a: SafeTensor<f32, [512, 512]> = SafeTensor::zeros();
know b: SafeTensor<f32, [256, 512]> = SafeTensor::zeros();

// ❌ Compile-time error: shape mismatch — [512,512] ≠ [256,512]
know c = a + b;

// ✅ Valid matmul: [512,512] × [512,256] → [512,256]
know d = matmul(a, SafeTensor::zeros::<f32, [512, 256]>());
```

### SafeTensor Operations

| Operation | Syntax | GEMM-lowered |
|-----------|--------|-------------|
| Matrix multiply | `matmul(a, b)` | Yes — single GEMM call |
| Element-wise add | `a + b` | No — fused loop |
| Element-wise multiply | `a * b` | No — fused loop |
| ReLU activation | `relu(a)` | No — vectorised |
| L2 normalise | `normalize(a)` | No — vectorised + reduce |
| Convolution | `conv2d(a, kernel)` | Yes — GEMM via im2col |
| Transpose | `a.T` | No — view (zero-copy) |
| Reshape | `a.reshape([N, M])` | No — view (zero-copy) |

---

## 6. Computation Graph

HoloLang allows explicit declaration of computation pipelines as graphs. Graphs are statically compiled — every edge is type-checked and every operation is lowered to the appropriate kernel before the session starts.

```hololang
graph BeamProcessingPipeline {
    input:  SafeTensor<f32, [512, 512]>     // raw sensor capture
    output: SafeTensor<f32, [512, 512]>     // normalised beam map

    steps:
        matmul(input, weight_matrix)         // [512,512]×[512,512] → [512,512]
        → relu                               // rectify negatives
        → normalize                          // L2-normalise each row
}
```

### Graph Properties

- **Lazy evaluation**: graphs do not execute until `run(graph, input)` is called.
- **Fused kernels**: adjacent compatible operations (e.g., matmul + relu) are fused into a single kernel dispatch.
- **Parallel branches**: graphs can declare parallel branches that execute concurrently on separate AERO actors.

```hololang
graph SensorFusionPipeline {
    input_a: SafeTensor<f32, [1, 5000]>    // SensorA (photodiode at 5kHz)
    input_b: SafeTensor<f32, [1, 1000]>    // SensorB (CCD at 1kHz)
    output:  SafeTensor<f32, [1, 512]>     // fused feature vector

    // Parallel preprocessing branches
    parallel {
        branch_a: resample(input_a, target: 512) → normalize
        branch_b: resample(input_b, target: 512) → normalize
    }

    // Merge and fuse
    → concat(branch_a, branch_b)           // [1, 1024]
    → matmul(_, fusion_weights)            // [1, 1024] × [1024, 512] → [1, 512]
    → relu
}
```

---

## 7. GEMM Acceleration

GEMM (General Matrix Multiplication) is the mathematical backbone of HoloLang's computation model. Any `matmul`, `conv2d`, or linear-algebra-heavy operation in a graph is automatically lowered to an optimised GEMM call dispatched to the best available accelerator.

### Accelerator Hierarchy

```
HoloLang graph op (matmul, conv2d, ...)
        │
        │  compile-time lowering
        ▼
GEMM kernel descriptor (M, N, K, dtype, layout)
        │
        │  runtime dispatch (DDAC-guided)
        ├─── GPU (cuBLAS / ROCm)         highest throughput
        ├─── Tensor Core (WMMA)          structured sparsity
        ├─── CPU AVX-512 (OpenBLAS)      fallback, no GPU
        └─── CPU NEON (ARM)              embedded, low-power
```

### GEMM Configuration

```hololang
@gemm(
    prefer:     "gpu",           // "gpu" | "tensor_core" | "cpu" | "auto"
    precision:  "f32",           // "f16" | "f32" | "f64" | "bf16"
    tile_size:  128,             // GEMM tile dimension
    async:      true,            // dispatch async, overlap with device I/O
)
graph AcceleratedPipeline { … }
```

### Calibration

Before the first session run, HoloLang performs a **GEMM calibration pass** — it benchmarks the available accelerators with representative problem sizes and builds a dispatch table that DDAC uses to route subsequent computations.

```hololang
@calibrate(
    warmup_iters: 10,
    bench_iters:  50,
    output:       "calibration.json",    // persisted, reused across sessions
)
```

---

## 8. MDI Canvas — Spatial Mesh Model

The MDI (Multi-Domain Interface) canvas is HoloLang's spatial model for projection systems. It divides the projection space into a mesh of tiles, each independently addressable and capable of carrying its own impulse cycle.

```hololang
canvas ProjectionSurface {
    mesh:       [4, 4]           // 4×4 grid of tiles (16 total)
    tile_size:  [128, 128]       // pixels per tile
    frame_rate: 60               // Hz
    depth_map:  true             // 3D depth channel per tile
}
```

### Tile Addressing

Tiles are addressed as `[row, col]` from the top-left corner. Tile operations can be applied individually or to ranges:

```hololang
// Apply an impulse to a single tile
canvas.tile([2, 3]).set_impulse(pulse_waveform)

// Apply to all tiles in row 1
canvas.row(1).set_brightness(0.8)

// Apply to a rectangular region
canvas.region([0,0], [2,2]).clear()
```

### Impulse Cycles

An impulse cycle is the timed sequence of beam operations applied to a tile within one frame period:

```hololang
impulse_cycle DebugSweep {
    duration: "1 frame"          // executes within one 60Hz frame
    tiles:    [[0,0], [0,1]]     // applies to 2 tiles (debug mode)

    steps:
        set_power(GreenLaser, 50.0)          // mW
        sweep(MirrorH, from: -5.0, to: 5.0) // horizontal sweep
        sample(SensorA, samples: 100)         // capture return
        set_power(GreenLaser, 0.0)            // laser off
}
```

### Mesh Scan Patterns

HoloLang provides built-in scan patterns for common projection strategies:

```hololang
scan_pattern FullSweep {
    direction: ScanDirection::BOUSTROPHEDON   // alternating line scan
    speed:     "max"                          // maximum galvo speed
    tiles:     canvas.all()
}
```

---

## 9. Communication Channels

HoloLang sessions can communicate with external systems through typed, effect-tracked channels. Each channel compiles to an AERO actor with the appropriate network effects.

### gRPC Channel

```hololang
channel BeamDataStream {
    type:     grpc
    proto:    "beam_data.proto"
    endpoint: "grpc://analytics.example.com:50051"
    tls:      true
    compress: "gzip"
}
```

### WebSocket Channel

```hololang
channel LiveMonitor {
    type:      websocket
    endpoint:  "wss://dashboard.example.com/ws/beam"
    heartbeat: "5s"
    reconnect: true
}
```

### Webhook Channel

```hololang
channel AlertHook {
    type:       webhook
    endpoint:   "https://alerts.example.com/v1/events"
    method:     POST
    auth:       bearer("${WEBHOOK_SECRET}")
    retry:      3
}
```

### REST API Channel

```hololang
channel ConfigAPI {
    type:       rest
    base_url:   "https://config.example.com/api/v2"
    auth:       api_key("${CONFIG_API_KEY}")
    timeout:    "2s"
}
```

### Sending and Receiving

```hololang
// Send beam snapshot over gRPC
BeamDataStream.send(BeamSnapshot {
    timestamp: Instant::now(),
    tensor:    beam_map,
    session:   @session.name,
})

// Receive configuration update
know config_update = ConfigAPI.get("/projection/config") ? ;
```

---

## 10. Enumerations and Directional Control

HoloLang supports integer-backed enumerations for type-safe directional and mode control:

```hololang
enum ScanDirection {
    LEFT_RIGHT   = 0,
    RIGHT_LEFT   = 1,
    BOUSTROPHEDON = 2,   // alternating: L→R, then R→L, repeat
    SERPENTINE   = 3,    // BOUSTROPHEDON with sub-line zig-zag
    SPIRAL_IN    = 4,    // outside-in spiral
    SPIRAL_OUT   = 5,    // inside-out spiral
    RANDOM       = 6,    // stochastic (dithering, anti-banding)
}

enum LaserModulation {
    CW      = 0,     // continuous wave
    PULSED  = 1,     // fixed-frequency pulses
    PWM     = 2,     // pulse-width modulated
    AM      = 3,     // amplitude modulated
}

enum CalibrationMode {
    NONE        = 0,
    GEOMETRIC   = 1,   // mirror angle offsets
    RADIOMETRIC = 2,   // power response curves
    FULL        = 3,   // geometric + radiometric
}
```

Enumerations are type-safe. Passing an integer literal where a `ScanDirection` is expected is a compile-time error.

---

## 11. Skill Tracking

Skills are named, versioned capabilities that a session declares and tracks. Skills bridge HoloLang's session model to AERO's capability system.

```hololang
skills {
    beam_calibration:   "1.2.0"
    sensor_fusion:      "2.0.1"
    ddac_adaptation:    "1.0.0"
    gemm_dispatch:      "3.1.0"
    grpc_streaming:     "1.5.2"
}
```

### Skill Resolution

Before a session starts, the AERO runtime verifies that every declared skill has a compatible implementation available in the framework. Skills that are incompatible (wrong version, missing dependency) produce a structured `SkillResolutionError` at startup — not a runtime crash.

### Skill Compatibility Checks

Skills check:
- **Version compatibility** (semver `^` rules),
- **Effect compatibility** (skill's declared effects ⊆ session's granted capabilities),
- **Device compatibility** (skill's required device types ⊆ session's declared devices).

### Linear Pull Agent Sessions

For Linear-driven delivery workflows, sessions can declare orchestrated agent requests using `@agent` blocks. This keeps `@`-addressed requests, instruction policy, and runtime enforcement in one place.

```hololang
linear_pull ProjectDelivery {
    source: "linear://project/AERO-42"
    mode:   "orchestrated"
}

@agent("warehouse_graveler")
instructions {
    intent: "transform-and-classify"
    enforce: [
        "require_skill:data_transform",
        "require_skill:data_classification",
        "deny_unmapped_targets",
        "emit_metrics:latency,throughput,error_rate"
    ]
    transport: "grpc+proto"
    topology:  "planes,lanes,controllers,shards"
}
```

The compiler validates that every enforced statement maps to declared skills/capabilities before session startup. Invalid mappings fail fast with structured diagnostics.

---

## 12. DocDirectory Generation

HoloLang generates a structured documentation directory from the session's declarations. The DocDirectory is a machine-readable JSON/Markdown artefact describing every device, graph, canvas, channel, and skill in the session.

```hololang
doc_directory {
    output:   "./docs/generated/"
    format:   ["markdown", "json"]
    include:  ["devices", "graphs", "channels", "skills", "enums"]
    version:  @session.name
}
```

### Generated Artefacts

```
docs/generated/
├── index.md               — session overview
├── devices/
│   ├── GreenLaser.md
│   ├── MirrorH.md
│   └── SensorA.md
├── graphs/
│   ├── BeamProcessingPipeline.md
│   └── SensorFusionPipeline.md
├── channels/
│   ├── BeamDataStream.md
│   └── LiveMonitor.md
├── skills/
│   └── skill_manifest.json
└── session.json           — machine-readable session descriptor
```

---

## 13. Pooled VM Runtime

For high-throughput scenarios (multiple concurrent projection zones, replicated computation kernels), HoloLang supports a pooled VM model — multiple AVM instances sharing a common world-type namespace.

```hololang
vm_pool ProjectionPool {
    replicas:     4              // 4 AVM instances
    affinity:     "numa"         // NUMA-aware placement
    kernel_share: true           // replicate graph kernels across instances
    load_balance: "work-steal"   // AERO work-stealing across pool
}
```

The pool is backed by AERO's existing work-stealing scheduler. HoloLang's addition is **kernel replication** — pre-loading compiled GEMM kernels into each replica's memory so the first dispatch has zero JIT overhead.

---

## 14. Complete System Example

```hololang
/*
 * full_system.hl – Complete HoloLang system
 *
 * Covers:
 *   - Multi-device orchestration (laser + 2 mirrors + 2 sensors)
 *   - SafeTensor bounds-checked beam data
 *   - Computation graph: matmul → relu → normalize pipeline
 *   - Pooled VM runtime with replicated kernels
 *   - MDI canvas with 4×4 mesh, tile impulse cycles
 *   - gRPC channel + WebSocket + webhook + REST API
 *   - Session lifecycle and skill tracking
 *   - DocDirectory generation
 *   - Enum-based directional control
 *   - Debug impulse cycle (2 tiles)
 */

@session("full-system-demo")

@ddac(
    probe_interval:  "50ms",
    adapt_threshold: 0.05,
    warmup_cycles:   3,
)

@gemm(prefer: "auto", precision: "f32", async: true)

@calibrate(output: "calibration.json")

// ─── DEVICES ─────────────────────────────────────────────────

device GreenLaser {
    type:          "solid-state-laser"
    wavelength_nm: 532
    max_power_mw:  150
    modulation:    "cw"
}

device RedLaser {
    type:          "diode-laser"
    wavelength_nm: 650
    max_power_mw:   80
    modulation:    "pulsed"
}

device MirrorH {
    type:    "galvanometer-mirror"
    axis:    "horizontal"
    min_deg: -30
    max_deg:  30
}

device MirrorV {
    type:    "galvanometer-mirror"
    axis:    "vertical"
    min_deg: -25
    max_deg:  25
}

device SensorA {
    type:           "photodiode"
    sample_rate_hz: 5000
}

device SensorB {
    type:           "ccd"
    sample_rate_hz: 1000
}

device_group BeamSystem {
    laser:    GreenLaser
    mirror_h: MirrorH
    mirror_v: MirrorV
}

// ─── ENUMS ───────────────────────────────────────────────────

enum ScanDirection {
    LEFT_RIGHT    = 0,
    RIGHT_LEFT    = 1,
    BOUSTROPHEDON = 2,
    SERPENTINE    = 3,
    SPIRAL_IN     = 4,
    SPIRAL_OUT    = 5,
    RANDOM        = 6,
}

enum CalibrationMode {
    NONE        = 0,
    GEOMETRIC   = 1,
    RADIOMETRIC = 2,
    FULL        = 3,
}

// ─── SAFE TENSORS ────────────────────────────────────────────

know beam_map:   SafeTensor<f32, [512, 512]> = SafeTensor::zeros();
know weight_mat: SafeTensor<f32, [512, 512]> = SafeTensor::load("weights/beam_v2.bin");
know fusion_w:   SafeTensor<f32, [1024, 512]> = SafeTensor::load("weights/fusion_v1.bin");

// ─── COMPUTATION GRAPHS ───────────────────────────────────────

graph BeamProcessingPipeline {
    input:  SafeTensor<f32, [512, 512]>
    output: SafeTensor<f32, [512, 512]>
    steps:
        matmul(input, weight_mat)
        → relu
        → normalize
}

graph SensorFusionPipeline {
    input_a: SafeTensor<f32, [1, 5000]>
    input_b: SafeTensor<f32, [1, 1000]>
    output:  SafeTensor<f32, [1, 512]>
    parallel {
        branch_a: resample(input_a, target: 512) → normalize
        branch_b: resample(input_b, target: 512) → normalize
    }
    → concat(branch_a, branch_b)
    → matmul(_, fusion_w)
    → relu
}

// ─── POOLED VM ───────────────────────────────────────────────

vm_pool ProjectionPool {
    replicas:     4
    affinity:     "numa"
    kernel_share: true
    load_balance: "work-steal"
}

// ─── MDI CANVAS ──────────────────────────────────────────────

canvas ProjectionSurface {
    mesh:       [4, 4]
    tile_size:  [128, 128]
    frame_rate: 60
    depth_map:  true
}

impulse_cycle DebugSweep {
    duration: "1 frame"
    tiles:    [[0, 0], [0, 1]]    // debug: 2 tiles only
    steps:
        set_power(GreenLaser, 50.0)
        sweep(MirrorH, from: -5.0, to: 5.0)
        sample(SensorA, samples: 100)
        set_power(GreenLaser, 0.0)
}

impulse_cycle FullScan {
    duration:  "1 frame"
    tiles:     ProjectionSurface.all()
    direction: ScanDirection::BOUSTROPHEDON
    steps:
        set_power(GreenLaser, 100.0)
        scan_pattern(MirrorH, MirrorV, direction: ScanDirection::BOUSTROPHEDON)
        sample(SensorA, samples: 500)
        sample(SensorB, samples: 100)
        set_power(GreenLaser, 0.0)
}

// ─── CHANNELS ────────────────────────────────────────────────

channel BeamDataStream {
    type:     grpc
    proto:    "beam_data.proto"
    endpoint: "grpc://analytics.example.com:50051"
    tls:      true
    compress: "gzip"
}

channel LiveMonitor {
    type:      websocket
    endpoint:  "wss://dashboard.example.com/ws/beam"
    heartbeat: "5s"
    reconnect: true
}

channel AlertHook {
    type:     webhook
    endpoint: "https://alerts.example.com/v1/events"
    method:   POST
    auth:     bearer("${WEBHOOK_SECRET}")
    retry:    3
}

channel ConfigAPI {
    type:     rest
    base_url: "https://config.example.com/api/v2"
    auth:     api_key("${CONFIG_API_KEY}")
    timeout:  "2s"
}

// ─── SKILLS ──────────────────────────────────────────────────

skills {
    beam_calibration: "1.2.0"
    sensor_fusion:    "2.0.1"
    ddac_adaptation:  "1.0.0"
    gemm_dispatch:    "3.1.0"
    grpc_streaming:   "1.5.2"
}

// ─── DOC DIRECTORY ───────────────────────────────────────────

doc_directory {
    output:  "./docs/generated/"
    format:  ["markdown", "json"]
    include: ["devices", "graphs", "channels", "skills", "enums"]
    version: @session.name
}

// ─── MAIN LOOP ───────────────────────────────────────────────

fn run() ! [BeamSystem, SensorA, SensorB, BeamDataStream, LiveMonitor, log, metrics] {
    // Calibrate on startup
    calibrate(CalibrationMode::FULL);

    loop {
        // Capture raw beam snapshot
        know raw_a = SensorA.sample(samples: 5000);
        know raw_b = SensorB.sample(samples: 1000);

        // Fuse sensor data
        know fused = run(SensorFusionPipeline, input_a: raw_a, input_b: raw_b);

        // Process beam map
        know processed = run(BeamProcessingPipeline, input: beam_map);

        // Run impulse cycle
        ProjectionSurface.run_cycle(FullScan);

        // Stream results
        BeamDataStream.send(BeamSnapshot {
            timestamp: Instant::now(),
            tensor:    processed,
            fused:     fused,
        });

        emit metrics::gauge("beam.peak_intensity", processed.max());
        emit metrics::gauge("sensor.fusion_score", fused.mean());
    }
}
```

---

## 15. Compiled AERO Output — Annotated

The following shows how selected HoloLang constructs map to AERO code after compilation:

### Device → World Type

```aero
// HoloLang: device GreenLaser { wavelength_nm: 532, max_power_mw: 150 }
// Compiled AERO:
world GreenLaserState {
    power_mw:      Float64,   // 0..=150 (enforced)
    wavelength_nm: u32,       // fixed: 532
    modulation:    LaserModulation,
    online:        bool,
}

// Device bounds compiled into a NewType wrapper:
struct PowerMw(f64);
impl PowerMw {
    fn new(v: f64) -> Result<Self, BoundsError> {
        if v >= 0.0 && v <= 150.0 { Ok(Self(v)) }
        else { Err(BoundsError::exceeded("max_power_mw", v, 150.0)) }
    }
}
```

### Graph → Pure Function + GEMM Dispatch

```aero
// HoloLang: graph BeamProcessingPipeline { matmul → relu → normalize }
// Compiled AERO:
fn beam_processing_pipeline(
    input:      SafeTensor<f32, Dim2<512, 512>>,
    weight_mat: SafeTensor<f32, Dim2<512, 512>>,
) -> SafeTensor<f32, Dim2<512, 512>> ! [gemm_dispatch] {
    know after_matmul = gemm::dispatch(input, weight_mat);  // GEMM kernel
    know after_relu   = kernels::relu(after_matmul);        // vectorised
    kernels::normalize_l2(after_relu)                       // vectorised
}
```

### Channel → Actor

```aero
// HoloLang: channel BeamDataStream { type: grpc … }
// Compiled AERO:
actor BeamDataStreamActor {
    fn loop(conn: GrpcConn<BeamDataProto>) ! [http, log, metrics] {
        know msg: BeamSnapshot = recv() ! [actor];
        know result = conn.send(msg.to_proto()) ? ;
        emit metrics::count("grpc.beam_data.sent", 1, { status: "ok" });
        self.loop(conn);
    }
}
```

---

*HoloLang Specification v1.0 — AERO Lang System*
