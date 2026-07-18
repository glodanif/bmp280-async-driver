#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CalibrationData {
    pub t1: u16,
    pub t2: i16,
    pub t3: i16,
    pub p1: u16,
    pub p2: i16,
    pub p3: i16,
    pub p4: i16,
    pub p5: i16,
    pub p6: i16,
    pub p7: i16,
    pub p8: i16,
    pub p9: i16,
}

impl CalibrationData {
    pub fn from_raw_bytes(raw: [u8; 24]) -> Self {
        Self {
            t1: u16::from_le_bytes([raw[0], raw[1]]),
            t2: i16::from_le_bytes([raw[2], raw[3]]),
            t3: i16::from_le_bytes([raw[4], raw[5]]),
            p1: u16::from_le_bytes([raw[6], raw[7]]),
            p2: i16::from_le_bytes([raw[8], raw[9]]),
            p3: i16::from_le_bytes([raw[10], raw[11]]),
            p4: i16::from_le_bytes([raw[12], raw[13]]),
            p5: i16::from_le_bytes([raw[14], raw[15]]),
            p6: i16::from_le_bytes([raw[16], raw[17]]),
            p7: i16::from_le_bytes([raw[18], raw[19]]),
            p8: i16::from_le_bytes([raw[20], raw[21]]),
            p9: i16::from_le_bytes([raw[22], raw[23]]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_bytes_to_calibration_types() {
        let data = CalibrationData::from_raw_bytes([0xFF; 24]);
        assert_eq!(data.t1, 65535);
        assert_eq!(data.t2, -1);
        assert_eq!(data.t3, -1);
        assert_eq!(data.p1, 65535);
        assert_eq!(data.p2, -1);
        assert_eq!(data.p3, -1);
        assert_eq!(data.p4, -1);
        assert_eq!(data.p5, -1);
        assert_eq!(data.p6, -1);
        assert_eq!(data.p7, -1);
        assert_eq!(data.p8, -1);
        assert_eq!(data.p9, -1);
    }

    #[test]
    fn calibration_types_to_raw_bytes() {
        let bytes = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ];
        let expected_calibration = CalibrationData {
            t1: 256,
            t2: 770,
            t3: 1284,
            p1: 1798,
            p2: 2312,
            p3: 2826,
            p4: 3340,
            p5: 3854,
            p6: 4368,
            p7: 4882,
            p8: 5396,
            p9: 5910,
        };

        assert_eq!(CalibrationData::from_raw_bytes(bytes), expected_calibration);
    }
}
