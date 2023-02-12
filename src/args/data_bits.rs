use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

impl DataBits {
    pub fn to_serialport_type(&self) -> tokio_serial::DataBits {
        match self {
            Self::Five => tokio_serial::DataBits::Five,
            Self::Six => tokio_serial::DataBits::Six,
            Self::Seven => tokio_serial::DataBits::Seven,
            Self::Eight => tokio_serial::DataBits::Eight,
        }
    }
}
