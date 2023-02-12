use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

impl FlowControl {
    pub fn to_serialport_type(&self) -> tokio_serial::FlowControl {
        match self {
            Self::None => tokio_serial::FlowControl::None,
            Self::Software => tokio_serial::FlowControl::Software,
            Self::Hardware => tokio_serial::FlowControl::Hardware,
        }
    }
}
