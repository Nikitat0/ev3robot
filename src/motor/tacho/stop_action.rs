use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopAction {
    Coast,
    Brake,
    Hold,
}

impl FromStr for StopAction {
    type Err = ParseStopActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "coast" => StopAction::Coast,
            "brake" => StopAction::Brake,
            "hold" => StopAction::Hold,
            _ => return Err(ParseStopActionError),
        })
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
#[error("provided string was not `coast`, `brake` or `hold`")]
pub struct ParseStopActionError;

impl Display for StopAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StopAction::Coast => "coast",
            StopAction::Brake => "brake",
            StopAction::Hold => "hold",
        }
        .fmt(f)
    }
}
