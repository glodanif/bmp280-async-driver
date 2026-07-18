/// Oversampling options
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Oversampling {
    /// X1
    X1 = 0b001,
    /// X2
    X2 = 0b010,
    /// X4
    X4 = 0b011,
    /// X8
    X8 = 0b100,
    /// X16
    X16 = 0b101,
}

/// Time stand-by options
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TimeStandby {
    /// 0.5ms
    Us500 = 0b000,
    /// 62.5ms
    Us62500 = 0b001,
    /// 125ms
    Ms125 = 0b010,
    /// 250ms
    Ms250 = 0b011,
    /// 500ms
    Ms500 = 0b100,
    /// 1000ms
    Ms1000 = 0b101,
    /// 2000ms
    Ms2000 = 0b110,
    /// 4000ms
    Ms4000 = 0b111,
}

/// IIR filter coefficient options
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FilterCoefficient {
    /// IIR filter disabled
    Off = 0b000,
    /// coefficient 2
    K2 = 0b001,
    /// coefficient 4
    K4 = 0b010,
    /// coefficient 8
    K8 = 0b011,
    /// coefficient 16
    K16 = 0b100,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Mode {
    Forced = 0b01,
    Normal = 0b11,
}
