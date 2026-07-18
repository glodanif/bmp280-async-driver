use crate::calibration_data::CalibrationData;
use crate::compensation::compensate_measurements;
use crate::register::Register;
use crate::sensor_data::SensorData;
use crate::{Bmp280Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Type-state marker: the device is in the measuring state
/// and reads temperature and pressure with [TimeStandby](crate::TimeStandby) interval,
/// ready to read measurements.
pub struct Normal {
    pub(crate) calibration_data: CalibrationData,
}

impl<T, D> Bmp280Device<T, D, Normal>
where
    T: I2c,
    D: DelayNs,
{
    /// Read temperature and pressure from the running chip
    ///
    /// # Errors
    ///
    /// I2C [Error::Bus] error if writing into register or reading from register fails
    pub async fn read(&mut self) -> Result<SensorData, Error<T::Error>> {
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
}
