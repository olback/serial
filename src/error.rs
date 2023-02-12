pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, giftwrap::Wrap)]
pub enum Error {
    Io(std::io::Error),
    Serialport(tokio_serial::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Serialport(e) => e.fmt(f),
        }
    }
}
