use std::fmt::{self, Debug, Display};

use derive_more::*;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Constructor, From, FromStr,
)]
pub struct TachoCounts {
    pub value: u32,
}

impl Debug for TachoCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TachoCounts").field(&self.value).finish()
    }
}

impl Display for TachoCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl From<TachoCounts> for u32 {
    fn from(TachoCounts { value }: TachoCounts) -> Self {
        value
    }
}
