use std::fmt::{self, Display};

use derive_more::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Constructor,
    From,
    FromStr,
)]
pub struct TachoCounts(pub u32);

impl TachoCounts {
    fn value(self) -> u32 {
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
