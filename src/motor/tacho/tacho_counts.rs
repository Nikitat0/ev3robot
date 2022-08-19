use std::fmt::{self, Display};

use derive_more::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, FromStr)]
pub struct TachoCounts(u32);

impl TachoCounts {
    pub fn new(value: u32) -> TachoCounts {
        TachoCounts(value)
    }

    pub fn value(self) -> u32 {
        self.into()
    }
}

impl Display for TachoCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value(), f)
    }
}

impl From<TachoCounts> for u32 {
    fn from(TachoCounts(value): TachoCounts) -> Self {
        value
    }
}
