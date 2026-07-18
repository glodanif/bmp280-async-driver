use crate::calibration_data::CalibrationData;
use crate::sensor_data::SensorData;

pub(crate) fn compensate_measurements(
    data_out: [u8; 6],
    calibration_data: &CalibrationData,
) -> SensorData {
    let (adc_p, adc_t) = raw_to_adc(data_out);
    let (t_fine, temperature_int) = compensate_t(calibration_data, adc_t);
    let pressure_int = compensate_p(calibration_data, t_fine, adc_p);
    SensorData {
        temperature_centi_c: temperature_int,
        pressure_q24_8: pressure_int,
    }
}

pub(crate) fn raw_to_adc(data_out: [u8; 6]) -> (i32, i32) {
    let adc_p =
        ((data_out[0] as i32) << 12) | ((data_out[1] as i32) << 4) | (data_out[2] as i32 >> 4);
    let adc_t =
        ((data_out[3] as i32) << 12) | ((data_out[4] as i32) << 4) | (data_out[5] as i32 >> 4);
    (adc_p, adc_t)
}

// Integer compensation formulas from the BMP280 datasheet, Appendix A.
pub(crate) fn compensate_t(calibration_data: &CalibrationData, adc_t: i32) -> (i32, i32) {
    let var1 =
        (((adc_t >> 3) - ((calibration_data.t1 as i32) << 1)) * calibration_data.t2 as i32) >> 11;
    let var2 = (((((adc_t >> 4) - (calibration_data.t1 as i32))
        * ((adc_t >> 4) - (calibration_data.t1 as i32)))
        >> 12)
        * calibration_data.t3 as i32)
        >> 14;
    let t_fine = var1 + var2;
    (t_fine, (t_fine * 5 + 128) >> 8)
}

pub(crate) fn compensate_p(calibration_data: &CalibrationData, t_fine: i32, adc_p: i32) -> u32 {
    let var1 = (t_fine as i64) - 128000;
    let var2 = var1 * var1 * (calibration_data.p6 as i64);
    let var2 = var2 + ((var1 * (calibration_data.p5 as i64)) << 17);
    let var2 = var2 + ((calibration_data.p4 as i64) << 35);
    let var1 = ((var1 * var1 * (calibration_data.p3 as i64)) >> 8)
        + ((var1 * (calibration_data.p2 as i64)) << 12);
    let var1 = ((((1i64) << 47) + var1) * (calibration_data.p1 as i64)) >> 33;
    if var1 == 0 {
        return 0;
    }
    let mut p = 1048576 - adc_p as i64;
    p = (((p << 31) - var2) * 3125) / var1;
    let var1 = ((calibration_data.p9 as i64) * (p >> 13) * (p >> 13)) >> 25;
    let var2 = ((calibration_data.p8 as i64) * p) >> 19;
    p = ((p + var1 + var2) >> 8) + ((calibration_data.p7 as i64) << 4);
    p as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden vectors from the Bosch BMP280 reference compensation example:
    // a fixed calibration + raw ADC readings with published expected outputs.
    const CALIBRATION: CalibrationData = CalibrationData {
        t1: 27504,
        t2: 26435,
        t3: -1000,
        p1: 36477,
        p2: -10685,
        p3: 3024,
        p4: 2855,
        p5: 140,
        p6: -7,
        p7: 15500,
        p8: -14600,
        p9: 6000,
    };
    const ADC_P: i32 = 415148; // raw pressure ADC reading
    const ADC_T: i32 = 519888; // raw temperature ADC reading
    const RAW_BYTES_INPUT: [u8; 6] = [0x65, 0x5A, 0xC0, 0x7E, 0xED, 0x00]; // 6-byte data block encoding ADC_P then ADC_T (MSB/LSB/XLSB per channel)
    const EXPECTED_P_Q24_8: u32 = 25767233; // Q24.8 Pa (÷256 = 100653.25 Pa)
    const EXPECTED_TEMP_CENTI_C: i32 = 2508; // 25.08 °C (value is °C × 100)

    #[test]
    fn temperature_matches_datasheet_example() {
        const EXPECTED_T_FINE: i32 = 128422; // intermediate fine-temperature term
        let (t_fine, temperature_int) = compensate_t(&CALIBRATION, ADC_T);
        assert_eq!(t_fine, EXPECTED_T_FINE);
        assert_eq!(temperature_int, EXPECTED_TEMP_CENTI_C);
    }

    #[test]
    fn pressure_matches_datasheet_example() {
        const T_FINE: i32 = 128422; // fine temp from the temperature example
        const EXPECTED_HPA: f32 = 1006.5325; // = EXPECTED_P_Q24_8 / 256 / 100
        let p_q24_8 = compensate_p(&CALIBRATION, T_FINE, ADC_P);
        let hpa: f32 = p_q24_8 as f32 / 256.0 / 100.0;
        assert_eq!(p_q24_8, EXPECTED_P_Q24_8);
        assert!((hpa - EXPECTED_HPA).abs() < 0.0001);
    }

    #[test]
    fn raw_to_adc_conversion() {
        let (adc_p, adc_t) = raw_to_adc(RAW_BYTES_INPUT);
        assert_eq!(adc_p, ADC_P);
        assert_eq!(adc_t, ADC_T);
    }

    #[test]
    fn measurements_compensation() {
        let sensor_data = compensate_measurements(RAW_BYTES_INPUT, &CALIBRATION);
        let expected = SensorData {
            temperature_centi_c: EXPECTED_TEMP_CENTI_C,
            pressure_q24_8: EXPECTED_P_Q24_8,
        };
        assert_eq!(sensor_data, expected);
    }
}
