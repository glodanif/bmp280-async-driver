use crate::calibration_data::CalibrationData;
use crate::compensation::compensate_measurements;
use crate::config::ForcedConfig;
use crate::register::Register;
use crate::sensor_data::SensorData;
use crate::{Bmp280Device, Error, STATUS_MEASURING};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Type-state marker: the device is in sleeping mode,
/// ready to measure, after which it will return to sleep.
pub struct Forced {
    pub(crate) calibration_data: CalibrationData,
    pub(crate) config: ForcedConfig,
}

impl<T, D> Bmp280Device<T, D, Forced>
where
    T: I2c,
    D: DelayNs,
{
    /// Measure temperature and pressure
    ///
    /// Triggers a measurement and waits for it to complete
    ///
    /// # Errors
    ///
    /// I2C [Error::Bus] error if writing into register or reading from register fails
    pub async fn measure(&mut self) -> Result<SensorData, Error<T::Error>> {
        self.i2c_bus
            .write(
                self.address,
                &[
                    Register::ControlMeasurements.address(),
                    self.state.config.control_measurements(),
                ],
            )
            .await
            .map_err(Error::Bus)?;

        self.wait_while_measuring().await?;

        let mut data_out: [u8; 6] = [0; 6];
        self.i2c_bus
            .write_read(
                self.address,
                &[Register::PressureData.address()],
                &mut data_out,
            )
            .await
            .map_err(Error::Bus)?;

        Ok(compensate_measurements(
            data_out,
            &self.state.calibration_data,
        ))
    }

    async fn wait_while_measuring(&mut self) -> Result<(), Error<T::Error>> {
        loop {
            let mut data_out = [0u8; 1];
            self.i2c_bus
                .write_read(self.address, &[Register::Status.address()], &mut data_out)
                .await
                .map_err(Error::Bus)?;
            if data_out[0] & STATUS_MEASURING == 0 {
                return Ok(());
            }
            self.delay.delay_ms(1).await;
        }
    }
}
