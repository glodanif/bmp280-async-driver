#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

//! # BMP280 async driver
//!
//! A `no_std` driver for the Bosch BMP280 pressure/temperature sensor,
//! built on `embedded-hal-async`. Bring your own bus — generic over any
//! `I2c` + `DelayNs`.
//!
//! Supports both forced and normal modes.
//!
//! Supports all available configuration options like oversampling, time stand-by, filter coefficient.
//!
//! ### Chip ID
//! Expected chip ID of an authentic mass production device is `0x58`. It's available as [EXPECTED_CHIP_ID]
//!
//! To check the chip ID or make sure your wiring is correct, you can use [chip_id](Bmp280Device::chip_id)
//!
//! ### Normal mode example:
//!
//! ```no_run
//! # use bmp280_driver::{Address, Bmp280Device, NormalConfig, Oversampling, TimeStandby, FilterCoefficient};
//! # use embedded_hal_mock::eh1::i2c::Mock as I2c;
//! # use embedded_hal_mock::eh1::delay::NoopDelay as Delay;
//! # async fn example() {
//! # let i2c = I2c::new(&[]);
//! # let delay = Delay::new();
//! let device = Bmp280Device::new(i2c, Address::Primary, delay);
//! let mut bmp280 = device
//!     .prepare()
//!     .await
//!     .unwrap()
//!     .into_normal(NormalConfig {
//!          pressure_oversampling: Oversampling::X8,
//!          temperature_oversampling: Oversampling::X2,
//!          time_standby: TimeStandby::Ms2000,
//!          filter_coefficient: FilterCoefficient::K4,
//!      })
//!     .await
//!     .unwrap();
//! let data = bmp280.read().await.unwrap();
//! # }
//! ```
//!
//! ### Forced mode:
//!
//! ```no_run
//! # use bmp280_driver::{Address, Bmp280Device, ForcedConfig, Oversampling};
//! # use embedded_hal_mock::eh1::i2c::Mock as I2c;
//! # use embedded_hal_mock::eh1::delay::NoopDelay as Delay;
//! # async fn example() {
//! # let i2c = I2c::new(&[]);
//! # let delay = Delay::new();
//! let device = Bmp280Device::new(i2c, Address::Secondary, delay);
//! let mut bmp280 = device
//!     .prepare()
//!     .await
//!     .unwrap()
//!     .into_forced(ForcedConfig {
//!          pressure_oversampling: Oversampling::X4,
//!          temperature_oversampling: Oversampling::X1,
//!      });
//! let data = bmp280.measure().await.unwrap();
//! # }
//! ```
//!
//! ### Optional features
//!
//! Enable the `defmt` feature for defmt::Format on Error/SensorData.

mod calibration_data;
mod compensation;
mod config;
mod parameters;
mod register;
mod sensor_data;
mod state;

use crate::calibration_data::CalibrationData;
use crate::register::Register;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

pub use crate::config::{ForcedConfig, NormalConfig};
pub use crate::parameters::{FilterCoefficient, Oversampling, TimeStandby};
pub use crate::sensor_data::SensorData;
pub use crate::state::{Forced, Normal, Ready, Unprepared};

/// Expected BMP280 chip id
pub const EXPECTED_CHIP_ID: u8 = 0x58;

const RESET_COMMAND: u8 = 0b1011_0110;
const STATUS_MEASURING: u8 = 0b0000_1000;

const RESET_STARTUP_MS: u32 = 2;

/// I2C device address
#[derive(Debug, Clone, Copy)]
pub enum Address {
    /// 0x76
    Primary,
    /// 0x77
    Secondary,
    /// Custom address
    Custom(u8),
}

/// I2C error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error, represent a failure when writing into register or reading from register
    Bus(E),
}

/// BMP280 device
///
/// In [Unprepared] state after creation,
/// moves into the [Ready] state after [prepare](Self::prepare),
/// moves into [Forced] or [Normal] after
/// [into_forced](Bmp280Device::into_forced) or [into_normal](Bmp280Device::into_normal),
/// moves into [Unprepared] after [reset](Self::reset)
pub struct Bmp280Device<T, D, S> {
    i2c_bus: T,
    address: u8,
    delay: D,
    state: S,
}

impl<T, D, S> Bmp280Device<T, D, S>
where
    T: I2c,
    D: DelayNs,
{
    /// Read the chip id from the device, compare to [EXPECTED_CHIP_ID] if you use an authentic chip
    ///
    /// # Errors
    ///
    /// I2C [Error::Bus] error if writing into register or reading from register fails
    pub async fn chip_id(&mut self) -> Result<u8, Error<T::Error>> {
        let mut data_out: [u8; 1] = [0u8; 1];
        let data_in = [Register::Id.address()];
        self.i2c_bus
            .write_read(self.address, &data_in, &mut data_out)
            .await
            .map_err(Error::Bus)?;
        Ok(data_out[0])
    }

    /// Soft-resets the chip (returning it to sleep), waits 2ms for power-on, and yields the device in [Unprepared] state.
    ///
    /// # Errors
    ///
    /// I2C [Error::Bus] error if writing into register fails
    pub async fn reset(mut self) -> Result<Bmp280Device<T, D, Unprepared>, Error<T::Error>> {
        self.soft_reset().await?;
        Ok(self.with_state(Unprepared))
    }

    async fn soft_reset(&mut self) -> Result<(), Error<T::Error>> {
        let data_in = [Register::Reset.address(), RESET_COMMAND];
        self.i2c_bus
            .write(self.address, &data_in)
            .await
            .map_err(Error::Bus)?;
        self.delay.delay_ms(RESET_STARTUP_MS).await;
        Ok(())
    }

    async fn read_calibration_data(&mut self) -> Result<CalibrationData, Error<T::Error>> {
        let mut out_data: [u8; 24] = [0; 24];
        self.i2c_bus
            .write_read(
                self.address,
                &[Register::CalibrationData.address()],
                &mut out_data,
            )
            .await
            .map_err(Error::Bus)?;
        Ok(CalibrationData::from_raw_bytes(out_data))
    }

    fn with_state<S2>(self, state: S2) -> Bmp280Device<T, D, S2> {
        Bmp280Device {
            i2c_bus: self.i2c_bus,
            address: self.address,
            delay: self.delay,
            state,
        }
    }
}
