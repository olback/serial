use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum StopBits {
    One,
    Two,
}

impl StopBits {
    pub fn to_serialport_type(&self) -> tokio_serial::StopBits {
        match self {
            Self::One => tokio_serial::StopBits::One,
            Self::Two => tokio_serial::StopBits::Two,
        }
    }
}
