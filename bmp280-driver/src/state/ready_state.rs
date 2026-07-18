use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use crate::{Bmp280Device, Error};
use crate::calibration_data::CalibrationData;
use crate::config::{ForcedConfig, NormalConfig};
use crate::state::forced_state::Forced;
use crate::state::normal_state::Normal;
use crate::register::Register;

/// Type-state marker: calibration loaded, ready to enter a measurement mode.
pub struct Ready {
    pub(crate) calibration_data: CalibrationData,
}

impl<T, D> Bmp280Device<T, D, Ready>
where
    T: I2c,
    D: DelayNs,
{
    /// Moves into `Forced` state - for the forced mode
    pub fn into_forced(self, forced_config: ForcedConfig) -> Bmp280Device<T, D, Forced> {
        let calibration = self.state.calibration_data;
        self.with_state(Forced {
            calibration_data: calibration,
            config: forced_config,
        })
    }

    /// Moves into `Normal` state - for the normal mode
    ///
    /// # Errors
    ///
    /// I2C [Error::Bus] error if writing into register or reading from register fails
    pub async fn into_normal(
        mut self,
        normal_config: NormalConfig,
    ) -> Result<Bmp280Device<T, D, Normal>, Error<T::Error>> {
        self.i2c_bus
            .write(
                self.address,
                &[Register::Config.address(), normal_config.config()],
            )
            .await
            .map_err(Error::Bus)?;

        self.i2c_bus
            .write(
                self.address,
                &[
                    Register::ControlMeasurements.address(),
                    normal_config.control_measurements(),
                ],
            )
            .await
            .map_err(Error::Bus)?;

        let calibration = self.state.calibration_data;
        Ok(self.with_state(Normal {
            calibration_data: calibration,
        }))
    }
}
