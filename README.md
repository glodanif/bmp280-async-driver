# bmp280-driver

A `no_std`, [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async) driver for the
Bosch **BMP280** pressure and temperature sensor.

Bring your own bus — the driver is generic over any async `I2c` + `DelayNs`, so it runs on any
HAL that implements `embedded-hal-async` 1.0 (Embassy, `esp-hal`, …) with no chip-specific code.

## Features

- **Both power modes** — forced (one-shot on demand) and normal (continuous free-running).
- **Full configuration** — per-channel oversampling, IIR filter coefficient, standby time.
- **Integer Bosch compensation** — the reference fixed-point math, verified against the datasheet;
  the core pulls in **no floating point** (readings are integer, with opt-in `f32` helpers).
- **Type-state API** — the compiler enforces the `Unprepared → Ready → Forced/Normal` flow;
  you can't read before configuring.
- **Optional `defmt`** — enable the `defmt` feature for `defmt::Format` on `Error` and `SensorData`.

## Usage

```rust
use bmp280_driver::{Address, Bmp280Device, NormalConfig, Oversampling, TimeStandby, FilterCoefficient};

let device = Bmp280Device::new(i2c, Address::Secondary, delay);

let mut bmp280 = device
    .prepare()
    .await?
    .into_normal(NormalConfig {
        pressure_oversampling: Oversampling::X16,
        temperature_oversampling: Oversampling::X2,
        time_standby: TimeStandby::Ms250,
        filter_coefficient: FilterCoefficient::K4,
    })
    .await?;

let data = bmp280.read().await?;
let celsius = data.temperature_celsius();
let hpa = data.pressure_hpa();
```

For single-shot measurements, use forced mode instead:

```rust
use bmp280_driver::{Bmp280Device, ForcedConfig, Oversampling};

let mut bmp280 = device
    .prepare()
    .await?
    .into_forced(ForcedConfig {
        pressure_oversampling: Oversampling::X4,
        temperature_oversampling: Oversampling::X1,
    });

let data = bmp280.measure().await?; // triggers a conversion, waits, and reads
```

`SensorData` stores raw integers (`temperature_centi_c`, `pressure_q24_8`); call
`temperature_celsius()` / `pressure_hpa()` when you want `f32`.

## Adding it as a dependency

```toml
[dependencies]
bmp280-driver = { git = "https://github.com/glodanif/bmp280-async-driver", tag = "v0.1.0" }
```

Enable defmt logging with `features = ["defmt"]`.

## Chip ID

An authentic mass-production BMP280 reports chip ID `0x58` (available as `EXPECTED_CHIP_ID`).
The driver does not verify identity automatically — sample parts report `0x56`/`0x57` and clones
vary — so call `chip_id()` and compare it yourself if you want to guard against a wrong or
mis-wired device.

## Example

[`nrf52-example`](nrf52-example) is a flashable Embassy application for the nRF52 DK
(probe-rs + defmt over RTT) that exercises the driver on real hardware.

## License

Licensed under the [MIT license](LICENSE).
