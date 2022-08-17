use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Normal,
    Inversed,
}

impl FromStr for Polarity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "normal" => Polarity::Normal,
            "inversed" => Polarity::Inversed,
            _ => bail!("provided string was not `normal` or `inversed`"),
        })
    }
}

impl Display for Polarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::Normal => "normal",
            Polarity::Inversed => "inversed",
        }
        .fmt(f)
    }
}
