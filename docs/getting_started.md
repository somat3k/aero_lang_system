# AERO Lang System — Getting Started

**Document Type:** Developer Guide  
**Version:** 1.0

---

## Prerequisites

| Requirement | Minimum Version | Notes |
|-------------|----------------|-------|
| Operating System | Linux (x86_64/arm64), macOS 12+, Windows 10+ | |
| RAM | 2 GB | 4 GB recommended for large projects |
| Disk | 500 MB | For toolchain and standard library |
| Internet | Required for initial install and package downloads | |

---

## 1. Install the AERO Toolchain

### 1.1 Automated Installer (Linux / macOS)

```bash
curl -fsSL https://get.aero-lang.dev | sh
```

This installs the AERO toolchain into `~/.aero/bin/` and adds it to your `PATH`.

### 1.2 Manual Download

Download the latest release package from the [AERO releases page](https://github.com/somat3k/aero_lang_system/releases) and extract it:

```bash
tar -xzf aero-toolchain-v1.0.0-linux-x86_64.tar.gz
export PATH="$PWD/aero-toolchain/bin:$PATH"
```

### 1.3 Verify Installation

```bash
aeroc --version
# Expected output:
# aeroc 1.0.0 (stable, 2026-04-04)

avm --version
# Expected output:
# avm 1.0.0
```

---

## 2. Create Your First Project

```bash
# Create a new application project
aeroc new hello-world
cd hello-world
```

This scaffolds the following structure:

```
hello-world/
├── Aero.toml
├── Aero.lock
└── src/
    └── main.aero
```

### `Aero.toml`

```toml
[package]
name    = "hello-world"
version = "0.1.0"
edition = "2026"
```

### `src/main.aero`

```aero
fn main() ! [log] {
    emit log::info("Hello, AERO!", {});
}
```

---

## 3. Run the Project

```bash
aeroc run
# Output:
# {"level":"info","msg":"Hello, AERO!","timestamp":"2026-04-04T14:21:13.218Z","actor":"main"}
```

To see pretty-printed logs during development:

```bash
AERO_LOG_FORMAT=pretty aeroc run
# Output:
# 2026-04-04 14:21:13 INFO  main — Hello, AERO!
```

---

## 4. Build and Test

```bash
# Type-check without producing output
aeroc check

# Build in debug mode
aeroc build

# Build in release mode (optimised)
aeroc build --release

# Run tests
aeroc test

# Run a specific test
aeroc test my_module::tests::my_test_name
```

---

## 5. A Real-World Example: Temperature Monitor

Let's build a small program that reads temperature from a sensor world, normalises the readings, and emits metrics.

### Step 1 — Define the World Type

Create `src/world.aero`:

```aero
/// A temperature observation from a physical or virtual sensor.
world Temperature {
    /// Temperature value in the reported unit.
    value: Float64,
    /// Unit of the temperature value.
    unit: TemperatureUnit,
    /// UTC timestamp of the observation.
    timestamp: Instant,
    /// Sensor identifier.
    sensor_id: String,
}

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}
```

### Step 2 — Write Domain Logic

Create `src/domain.aero`:

```aero
use crate::world::{Temperature, TemperatureUnit};

/// Converts any temperature reading to Celsius.
/// This is a pure function — no effects required.
pub fn normalise_to_celsius(reading: Temperature) -> Temperature {
    know celsius_value = match reading.unit {
        TemperatureUnit::Celsius    => reading.value,
        TemperatureUnit::Fahrenheit => (reading.value - 32.0) * 5.0 / 9.0,
        TemperatureUnit::Kelvin     => reading.value - 273.15,
    };

    Temperature {
        value: celsius_value,
        unit: TemperatureUnit::Celsius,
        ..reading
    }
}

/// Returns true if the temperature exceeds the alert threshold.
pub fn is_critical(reading: &Temperature, threshold_celsius: Float64) -> bool {
    reading.value > threshold_celsius
}
```

### Step 3 — Wire Up the Main Program

Update `src/main.aero`:

```aero
use crate::domain::{normalise_to_celsius, is_critical};

const ALERT_THRESHOLD_CELSIUS: Float64 = 80.0;

fn main() ! [temperature_sensor, log, metrics] {
    loop {
        know raw = temperature_sensor.observe();
        know normalised = normalise_to_celsius(raw);

        emit log::debug("observation", {
            sensor_id: normalised.sensor_id,
            value_celsius: normalised.value,
        });

        emit log::metric("temperature.celsius", normalised.value, {
            sensor: normalised.sensor_id,
        });

        if is_critical(&normalised, ALERT_THRESHOLD_CELSIUS) {
            emit log::warn("temperature critical", {
                sensor_id: normalised.sensor_id,
                value: normalised.value,
                threshold: ALERT_THRESHOLD_CELSIUS,
            });
        }

        sleep(Duration::seconds(5));
    }
}
```

### Step 4 — Add a World Adapter in `Aero.toml`

```toml
[package]
name    = "temperature-monitor"
version = "0.1.0"
edition = "2026"

[capabilities]
network = ["outbound:https"]

[dependencies]
aero-http = "1.2"

[world.temperature_sensor]
adapter = "HttpJsonAdapter"
url     = "https://my-sensor-api.example.com/v1/temperature"
poll_interval = "5s"
```

### Step 5 — Run with the Mock Adapter in Tests

Create `tests/domain_test.aero`:

```aero
use aero_test::*;
use crate::domain::{normalise_to_celsius, is_critical};
use crate::world::{Temperature, TemperatureUnit};

#[test]
fn test_fahrenheit_to_celsius() {
    know reading = Temperature {
        value: 212.0,
        unit: TemperatureUnit::Fahrenheit,
        timestamp: Instant::now(),
        sensor_id: "test-sensor".to_string(),
    };
    know normalised = normalise_to_celsius(reading);
    assert_eq!(normalised.value, 100.0);
    assert_eq!(normalised.unit, TemperatureUnit::Celsius);
}

#[test]
fn test_kelvin_to_celsius() {
    know reading = Temperature {
        value: 373.15,
        unit: TemperatureUnit::Kelvin,
        timestamp: Instant::now(),
        sensor_id: "test-sensor".to_string(),
    };
    know normalised = normalise_to_celsius(reading);
    assert!((normalised.value - 100.0).abs() < 0.001);
}

#[test]
fn test_critical_threshold() {
    know hot = Temperature {
        value: 85.0,
        unit: TemperatureUnit::Celsius,
        timestamp: Instant::now(),
        sensor_id: "test-sensor".to_string(),
    };
    know cool = Temperature {
        value: 60.0,
        unit: TemperatureUnit::Celsius,
        timestamp: Instant::now(),
        sensor_id: "test-sensor".to_string(),
    };
    assert!(is_critical(&hot, 80.0));
    assert!(!is_critical(&cool, 80.0));
}
```

Run the tests:

```bash
aeroc test
# 3 tests passed in 12ms
```

---

## 6. Explore Further

Now that you have a working project, explore the rest of the documentation:

| Document | What You'll Learn |
|----------|------------------|
| [Architecture](./architecture.md) | How the AVM, compiler, and runtime fit together |
| [Design Principles](./design_principles.md) | How to write idiomatic AERO code |
| [API Reference](./api_reference.md) | Every built-in type, function, and effect |
| [Roadmap](./roadmap.md) | What features are coming and when |
| [Glossary](./glossary.md) | Definitions of AERO-specific terms |

---

## 7. Getting Help

- **GitHub Issues:** https://github.com/somat3k/aero_lang_system/issues
- **Discussions:** https://github.com/somat3k/aero_lang_system/discussions
- **Documentation:** https://docs.aero-lang.dev (coming soon)

---

*AERO Lang System Getting Started Guide v1.0*
