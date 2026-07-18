/// Sensor data structure with integer temperature and pressure values
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorData {
    /// Temperature is stored in °C × 100
    pub temperature_centi_c: i32,
    /// Pressure is stored in Q24.8 Pa
    pub pressure_q24_8: u32,
}

impl SensorData {
    /// Returns a float value of temperature in °C
    pub fn temperature_celsius(&self) -> f32 {
        self.temperature_centi_c as f32 / 100.0
    }

    /// Returns a float value of pressure in hPa
    pub fn pressure_hpa(&self) -> f32 {
        self.pressure_q24_8 as f32 / 256.0 / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: SensorData = SensorData {
        temperature_centi_c: 2508, // 25.08 °C (value is °C × 100)
        pressure_q24_8: 25767233,  // Q24.8 Pa (÷256 = 100653.25 Pa)
    };

    #[test]
    fn celsius_calculation() {
        assert!((DATA.temperature_celsius() - 25.08).abs() < 0.001);
    }

    #[test]
    fn hpa_calculation() {
        assert!((DATA.pressure_hpa() - 1006.5325).abs() < 0.0001);
    }
}
