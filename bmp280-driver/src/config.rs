use crate::parameters::{FilterCoefficient, Mode, Oversampling, TimeStandby};

/// Configuration for the forced mode
#[derive(Debug, Clone, Copy)]
pub struct ForcedConfig {
    /// Pressure oversampling x1-x16
    pub pressure_oversampling: Oversampling,
    /// Temperature oversampling x1-x16
    pub temperature_oversampling: Oversampling,
}

impl ForcedConfig {
    pub(crate) const fn control_measurements(self) -> u8 {
        control_measurements_bits(
            self.temperature_oversampling,
            self.pressure_oversampling,
            Mode::Forced,
        )
    }
}

impl Default for ForcedConfig {
    fn default() -> Self {
        Self {
            pressure_oversampling: Oversampling::X1,
            temperature_oversampling: Oversampling::X1,
        }
    }
}

/// Configuration for the normal mode
#[derive(Debug, Clone, Copy)]
pub struct NormalConfig {
    /// Pressure oversampling x1-x16
    pub pressure_oversampling: Oversampling,
    /// Temperature oversampling x1-x16
    pub temperature_oversampling: Oversampling,
    /// Time stand-by 0.5ms-4000ms
    pub time_standby: TimeStandby,
    /// IIR Filter coefficient
    pub filter_coefficient: FilterCoefficient,
}

impl NormalConfig {
    pub(crate) const fn control_measurements(self) -> u8 {
        control_measurements_bits(
            self.temperature_oversampling,
            self.pressure_oversampling,
            Mode::Normal,
        )
    }

    pub(crate) const fn config(self) -> u8 {
        config_bits(self.time_standby, self.filter_coefficient)
    }
}

impl Default for NormalConfig {
    fn default() -> Self {
        Self {
            pressure_oversampling: Oversampling::X1,
            temperature_oversampling: Oversampling::X1,
            time_standby: TimeStandby::Ms250,
            filter_coefficient: FilterCoefficient::K4,
        }
    }
}

const fn control_measurements_bits(
    temperature: Oversampling,
    pressure: Oversampling,
    mode: Mode,
) -> u8 {
    ((temperature as u8) << 5) | ((pressure as u8) << 2) | (mode as u8)
}

const fn config_bits(time_standby: TimeStandby, filter_coefficient: FilterCoefficient) -> u8 {
    ((time_standby as u8) << 5) | ((filter_coefficient as u8) << 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORCED_CONFIG: ForcedConfig = ForcedConfig {
        pressure_oversampling: Oversampling::X4,
        temperature_oversampling: Oversampling::X8,
    };
    const NORMAL_CONFIG: NormalConfig = NormalConfig {
        pressure_oversampling: Oversampling::X4,
        temperature_oversampling: Oversampling::X8,
        time_standby: TimeStandby::Ms2000,
        filter_coefficient: FilterCoefficient::K8,
    };

    #[test]
    fn control_measurements_bits_forming_forced() {
        let bits = FORCED_CONFIG.control_measurements();
        assert_eq!(bits, 0b10001101);
    }

    #[test]
    fn control_measurements_bits_forming_normal() {
        let bits = NORMAL_CONFIG.control_measurements();
        assert_eq!(bits, 0b10001111);
    }

    #[test]
    fn config_bits_forming() {
        let bits = NORMAL_CONFIG.config();
        assert_eq!(bits, 0b11001100);
    }
}
