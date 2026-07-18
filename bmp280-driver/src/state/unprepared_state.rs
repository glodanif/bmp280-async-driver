use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use crate::{Address, Bmp280Device, Error};
use crate::state::ready_state::Ready;

/// Type-state marker: a device instance is created, but not prepared.
pub struct Unprepared;

impl<T, D> Bmp280Device<T, D, Unprepared>
where
    T: I2c,
    D: DelayNs,
{
    /// Create a new instance of `Bmp280Device` in `Unprepared` mode
    pub fn new(i2c_bus: T, address: Address, delay: D) -> Self {
        let address = match address {
            Address::Primary => 0x76,
            Address::Secondary => 0x77,
            Address::Custom(address) => address,
        };
        Self {
            i2c_bus,
            address,
            delay,
            state: Unprepared,
        }
    }

    /// Prepare the device for use
    ///
    /// Moves the device into `Ready` state
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if the device cannot be prepared
    /// due to writing into register or reading from register failures
    pub async fn prepare(mut self) -> Result<Bmp280Device<T, D, Ready>, Error<T::Error>> {
        self.soft_reset().await?;
        let calibration_data = self.read_calibration_data().await?;
        Ok(self.with_state(Ready { calibration_data }))
    }
}
