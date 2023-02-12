#[derive(Debug, Clone)]
pub struct Baud(u32);

impl Baud {
    pub const fn new(baud: u32) -> Self {
        Self(baud)
    }
}

impl core::ops::Deref for Baud {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Display for Baud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for Baud {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Baud(s.parse::<u32>()?))
    }
}
