use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Normal,
    Inversed,
}

impl FromStr for Polarity {
    type Err = ParsePolarityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "normal" => Polarity::Normal,
            "inversed" => Polarity::Inversed,
            _ => return Err(ParsePolarityError),
        })
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
#[error("provided string was not `normal` or `inversed`")]
pub struct ParsePolarityError;

impl Display for Polarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::Normal => "normal",
            Polarity::Inversed => "inversed",
        }
        .fmt(f)
    }
}
