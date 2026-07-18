#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum Register {
    CalibrationData = 0x88,
    Id = 0xD0,
    Reset = 0xE0,
    Status = 0xF3,
    ControlMeasurements = 0xF4,
    Config = 0xF5,
    PressureData = 0xF7,
}

impl Register {
    pub const fn address(self) -> u8 {
        self as u8
    }
}
