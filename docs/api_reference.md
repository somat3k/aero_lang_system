# AERO Lang System — API Reference

**Document Type:** Language & Runtime API Reference  
**Version:** 1.0  
**Edition:** 2026

---

## Table of Contents

1. [Primitive Types](#1-primitive-types)
2. [Core Types](#2-core-types)
3. [Effect System](#3-effect-system)
4. [World Types](#4-world-types)
5. [Standard Library Modules](#5-standard-library-modules)
6. [Built-in Functions](#6-built-in-functions)
7. [Resilience Primitives](#7-resilience-primitives)
8. [Actor Primitives](#8-actor-primitives)
9. [Telemetry API](#9-telemetry-api)
10. [Testing API](#10-testing-api)
11. [Compiler Attributes](#11-compiler-attributes)

---

## 1. Primitive Types

| Type | Description | Range / Notes |
|------|-------------|---------------|
| `bool` | Boolean | `true` or `false` |
| `i8` | Signed 8-bit integer | -128 to 127 |
| `i16` | Signed 16-bit integer | -32 768 to 32 767 |
| `i32` | Signed 32-bit integer | -2³¹ to 2³¹−1 |
| `i64` | Signed 64-bit integer | -2⁶³ to 2⁶³−1 |
| `i128` | Signed 128-bit integer | -2¹²⁷ to 2¹²⁷−1 |
| `u8` | Unsigned 8-bit integer | 0 to 255 |
| `u16` | Unsigned 16-bit integer | 0 to 65 535 |
| `u32` | Unsigned 32-bit integer | 0 to 2³²−1 |
| `u64` | Unsigned 64-bit integer | 0 to 2⁶⁴−1 |
| `u128` | Unsigned 128-bit integer | 0 to 2¹²⁸−1 |
| `f32` | 32-bit IEEE 754 float | ~7 decimal digits |
| `f64` | 64-bit IEEE 754 float | ~15 decimal digits |
| `Float64` | Alias for `f64` | Preferred in domain code |
| `char` | Unicode scalar value | U+0000 to U+D7FF, U+E000 to U+10FFFF |
| `str` | UTF-8 string slice | Immutable, borrowed |
| `String` | Owned UTF-8 string | Heap-allocated, mutable |
| `()` | Unit type | Zero-size; used for "no return value" |
| `!` | Never type | A computation that never returns |

---

## 2. Core Types

### `Result<T, E>`

Represents either a successful value (`Ok(T)`) or an error (`Err(E)`).

```aero
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Key methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `map` | `(T → U) → Result<U, E>` | Transform the `Ok` value |
| `map_err` | `(E → F) → Result<T, F>` | Transform the `Err` value |
| `and_then` | `(T → Result<U, E>) → Result<U, E>` | Chain fallible operations |
| `unwrap_or` | `T → T` | Return value or default |
| `unwrap_or_else` | `(E → T) → T` | Return value or compute default |
| `is_ok` | `() → bool` | True if `Ok` |
| `is_err` | `() → bool` | True if `Err` |
| `?` operator | (language syntax) | Propagate `Err` to caller |

---

### `Option<T>`

Represents an optional value: either `Some(T)` or `None`.

```aero
enum Option<T> {
    Some(T),
    None,
}
```

**Key methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `map` | `(T → U) → Option<U>` | Transform the inner value |
| `and_then` | `(T → Option<U>) → Option<U>` | Chain optional operations |
| `unwrap_or` | `T → T` | Return value or default |
| `filter` | `(T → bool) → Option<T>` | Keep `Some` only if predicate holds |
| `is_some` | `() → bool` | True if `Some` |
| `is_none` | `() → bool` | True if `None` |
| `ok_or` | `E → Result<T, E>` | Convert to `Result` |

---

### `Vec<T>`

A heap-allocated, growable sequence of values of type `T`.

**Key methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `Vec::new()` | `() → Vec<T>` | Create an empty vector |
| `Vec::with_capacity(n)` | `usize → Vec<T>` | Pre-allocate capacity |
| `push` | `T → ()` | Append an element |
| `pop` | `() → Option<T>` | Remove and return last element |
| `len` | `() → usize` | Number of elements |
| `is_empty` | `() → bool` | True if zero elements |
| `get` | `usize → Option<&T>` | Borrow element by index |
| `iter` | `() → Iterator<&T>` | Iterate by reference |
| `map` | `(T → U) → Vec<U>` | Transform all elements |
| `filter` | `(T → bool) → Vec<T>` | Keep matching elements |
| `fold` | `(B, (B, T) → B) → B` | Reduce to a single value |

---

### `HashMap<K, V>`

A hash map from keys of type `K` to values of type `V`.

**Key methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `HashMap::new()` | `() → HashMap<K, V>` | Create an empty map |
| `insert` | `(K, V) → Option<V>` | Insert; returns old value if present |
| `get` | `&K → Option<&V>` | Look up a value by key |
| `remove` | `&K → Option<V>` | Remove a key-value pair |
| `contains_key` | `&K → bool` | Check key existence |
| `len` | `() → usize` | Number of entries |
| `iter` | `() → Iterator<(&K, &V)>` | Iterate over key-value pairs |
| `entry` | `K → Entry<K, V>` | Ergonomic insert-or-update API |

---

### `Instant`

A point in time with nanosecond resolution (UTC).

```aero
know now = Instant::now();
know later = now + Duration::seconds(10);
know elapsed: Duration = later - now;
```

---

### `Duration`

A length of time.

| Constructor | Example |
|-------------|---------|
| `Duration::nanoseconds(n)` | `Duration::nanoseconds(500)` |
| `Duration::microseconds(n)` | `Duration::microseconds(100)` |
| `Duration::milliseconds(n)` | `Duration::milliseconds(250)` |
| `Duration::seconds(n)` | `Duration::seconds(30)` |
| `Duration::minutes(n)` | `Duration::minutes(5)` |
| `Duration::hours(n)` | `Duration::hours(1)` |

---

## 3. Effect System

### Declaring Effects

Effects are declared in function signatures after the return type using `!`:

```aero
fn my_fn(arg: T) -> R ! [effect1, effect2] { … }
```

### Built-in Effects

| Effect | Description |
|--------|-------------|
| `log` | Emit structured log events |
| `metrics` | Emit numeric metrics |
| `trace` | Participate in distributed traces |
| `http` | Make or receive HTTP requests |
| `filesystem` | Read or write the local filesystem |
| `time` | Observe the current time or sleep |
| `rand` | Generate random numbers |
| `env` | Read environment variables |
| `io` | Generic I/O (subsumes `http`, `filesystem`) |
| `world<W>` | Observe or emit to world `W` |
| `actor` | Spawn, send to, or receive from actors |

### Effect Propagation

Effects propagate upward through call stacks. If `f` calls `g` which has effect `E`, then `f` must declare `E` (or handle it at a boundary):

```aero
fn g() -> () ! [log] { emit log::info("hi", {}); }

fn f() -> () ! [log] {  // must include 'log' because g() requires it
    g();
}
```

### Effect Handlers

Effects can be handled at explicit boundaries using `handle`:

```aero
know result = handle log with TestLogSink::new() {
    my_function_that_logs()
};
```

---

## 4. World Types

### Declaring a World Type

```aero
world MyWorld {
    field_name: FieldType,
    // …
    
    #[consistency = "eventual"]   // or "strong", "causal"
    // optional consistency annotation; default: "eventual"
}
```

### World Type Methods (Compiler-Generated)

| Method | Signature | Description |
|--------|-----------|-------------|
| `observe()` | `() → MyWorld ! [world<MyWorld>]` | Read the current world state |
| `emit(delta)` | `MyWorld::Delta → Result<(), WorldError> ! [world<MyWorld>]` | Push a change to the world |
| `subscribe()` | `() → Stream<WorldChanged<MyWorld>> ! [world<MyWorld>]` | Stream of change events |

### `WorldChanged<W>`

```aero
struct WorldChanged<W: World> {
    previous: W,
    current:  W,
    delta:    W::Delta,
    severity: DriftSeverity,
    observed_at: Instant,
}

enum DriftSeverity {
    Minor,
    Major,
    Critical,
}
```

---

## 5. Standard Library Modules

### `std::collections`

| Type | Description |
|------|-------------|
| `Vec<T>` | Growable array |
| `HashMap<K, V>` | Hash map |
| `HashSet<T>` | Hash set |
| `BTreeMap<K, V>` | Sorted map (B-tree) |
| `BTreeSet<T>` | Sorted set (B-tree) |
| `VecDeque<T>` | Double-ended queue |
| `NonEmpty<T>` | Guaranteed non-empty collection |

### `std::string`

| Function | Signature | Description |
|----------|-----------|-------------|
| `format!` | `(template, …args) → String` | Format a string |
| `String::from` | `&str → String` | Convert string slice to owned |
| `String::parse::<T>` | `() → Result<T, ParseError>` | Parse string to type `T` |

### `std::io`

| Function | Signature | Description |
|----------|-----------|-------------|
| `read_line` | `() → Result<String, IoError> ! [io]` | Read a line from stdin |
| `write` | `&str → Result<(), IoError> ! [io]` | Write to stdout |
| `read_file` | `&str → Result<String, IoError> ! [filesystem]` | Read entire file |
| `write_file` | `(&str, &str) → Result<(), IoError> ! [filesystem]` | Write entire file |

### `std::net`

See the `aero-http` package for the HTTP client/server.  
For raw TCP/UDP, use `std::net::TcpStream`, `std::net::UdpSocket`.

### `std::time`

| Function/Type | Description |
|---------------|-------------|
| `Instant::now()` | Current UTC instant |
| `sleep(Duration)` | Suspend current actor for `Duration` |
| `timeout<D>(expr)` | Execute `expr` with a deadline |

---

## 6. Built-in Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `panic!(msg)` | `&str → !` | Abort with programmer-error message |
| `assert!(cond)` | `bool → ()` | Panic if `cond` is false |
| `assert_eq!(a, b)` | `(T, T) → ()` | Panic if `a != b` |
| `dbg!(expr)` | `T → T` | Print debug representation and return value |
| `todo!()` | `() → !` | Mark unfinished code; panics at runtime |
| `unreachable!()` | `() → !` | Assert a code path is unreachable |
| `std::mem::size_of::<T>()` | `() → usize` | Size of type `T` in bytes |
| `std::mem::swap(a, b)` | `(&mut T, &mut T) → ()` | Swap two mutable references |

---

## 7. Resilience Primitives

### `retry<N>(expr)`

Retry `expr` up to `N` times with exponential back-off (base: 100ms, max: 30s, jitter: ±10%).

```aero
know result = retry<3>(database.query("SELECT …")) ! [db, time];
// Type: Result<QueryResult, RetryExhausted<DbError>>
```

### `circuit<T>(expr, fallback)`

Execute `expr` through a circuit breaker. If the circuit trips (error rate > 50% over 10 requests), returns `fallback` instead of calling `expr`.

```aero
know price = circuit(
    pricing_service.get_price(sku),
    fallback: Price::cached(sku),
) ! [http, log];
```

### `timeout<D>(expr)`

Execute `expr`, returning `Err(Timeout)` if it does not complete within duration `D`.

```aero
know result = timeout<Duration::milliseconds(500)>(
    external_api.fetch(request)
) ! [http];
// Type: Result<ApiResponse, Timeout | ApiError>
```

---

## 8. Actor Primitives

### `spawn(f)`

Spawn a new actor running function `f`. Returns an `ActorRef<T>` where `T` is the message type `f` accepts.

```aero
know worker: ActorRef<WorkItem> = spawn(worker_loop) ! [actor];
```

### `send(ref, msg)`

Send a message to an actor. Returns immediately (non-blocking).

```aero
send(worker, WorkItem { data: payload }) ! [actor];
```

### `recv()`

Receive the next message from the current actor's mailbox. Suspends until a message is available.

```aero
know msg: MyMessage = recv() ! [actor];
```

### `link(ref)`

Link the current actor to `ref`. If either actor terminates abnormally, the other receives an `Exit` signal.

```aero
link(worker) ! [actor];
```

### `monitor(ref)`

Monitor `ref`. Receive a `Down` message if `ref` terminates (without linking fate).

```aero
know monitor_ref = monitor(worker) ! [actor];
```

---

## 9. Telemetry API

All telemetry is emitted using the `emit` keyword and requires the corresponding effect.

### Structured Logs

```aero
emit log::trace("message", { key: value, … });   // requires: log
emit log::debug("message", { key: value, … });   // requires: log
emit log::info("message",  { key: value, … });   // requires: log
emit log::warn("message",  { key: value, … });   // requires: log
emit log::error("message", { key: value, … });   // requires: log
```

### Metrics

```aero
// Counter (monotonically increasing)
emit log::count("http.requests", 1, { method: "GET", status: 200 });

// Gauge (current value)
emit log::gauge("queue.depth", queue.len(), { queue: "orders" });

// Histogram (distribution of values)
emit log::histogram("http.latency_ms", elapsed.as_millis(), { route: "/api/v1/users" });
```

### Distributed Traces

```aero
// Manual span (usually automatic at actor boundaries)
know span = trace::span("database.query") ! [trace];
defer span.end();

emit trace::event("query.start", { sql: query }) ! [trace];
know result = database.execute(query) ! [db];
emit trace::event("query.end", { rows: result.len() }) ! [trace];
```

---

## 10. Testing API

### `aero_test` Module

```aero
use aero_test::*;
```

| Macro / Function | Description |
|-----------------|-------------|
| `#[test]` | Mark a function as a test case |
| `#[test(async)]` | Mark an async test case |
| `assert_eq!(a, b)` | Fail test if `a != b` |
| `assert_ne!(a, b)` | Fail test if `a == b` |
| `assert!(cond)` | Fail test if `cond` is false |
| `assert_matches!(expr, pattern)` | Fail test if `expr` doesn't match `pattern` |
| `capture_telemetry()` | Returns a handler that captures log/metric events for assertion |
| `mock_world::<W>()` | Returns a `MockAdapter<W>` for scripting world observations |
| `assert_log_contains!(logs, msg)` | Assert that captured logs contain `msg` |
| `assert_metric!(logs, name, value)` | Assert that a metric was emitted with given value |

### Example Test

```aero
use aero_test::*;

#[test]
fn test_with_mock_world() {
    know mock = mock_world::<Temperature>();
    mock.enqueue(Temperature { value: 100.0, unit: Celsius, … });
    mock.enqueue(Temperature { value: 200.0, unit: Celsius, … });

    know telemetry = capture_telemetry();

    handle world<Temperature> with mock,
          log with telemetry {
        process_readings();
    };

    assert_metric!(telemetry, "temperature.celsius", 100.0);
    assert_log_contains!(telemetry, "observation");
}
```

---

## 11. Compiler Attributes

| Attribute | Target | Description |
|-----------|--------|-------------|
| `#[derive(Debug)]` | struct, enum | Auto-generate debug representation |
| `#[derive(Clone)]` | struct, enum | Auto-generate `clone()` method |
| `#[derive(Eq, Hash)]` | struct, enum | Enable use as `HashMap` key |
| `#[serde]` | struct, enum | Enable JSON serialisation / deserialisation |
| `#[proto]` | struct, enum | Enable Protocol Buffers serialisation |
| `#[sql(table = "…")]` | struct | Map struct to SQL table |
| `#[validated]` | struct field | Apply validation rules on construction |
| `#[sensitive(class)]` | struct field | Mark field as sensitive data |
| `#[deprecated(since = "…")]` | any | Emit deprecation warning on use |
| `#[inline]` | fn | Hint to compiler to inline this function |
| `#[cold]` | fn | Hint that this function is rarely called |
| `#[test]` | fn | Mark as a test function |
| `#[bench]` | fn | Mark as a benchmark function |

---

*AERO Lang System API Reference v1.0*
