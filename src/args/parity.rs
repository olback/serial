use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Parity {
    None,
    Odd,
    Even,
}

impl Parity {
    pub fn to_serialport_type(&self) -> tokio_serial::Parity {
        match self {
            Self::None => tokio_serial::Parity::None,
            Self::Odd => tokio_serial::Parity::Odd,
            Self::Even => tokio_serial::Parity::Even,
        }
    }
}
