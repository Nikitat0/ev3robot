use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopAction {
    Coast,
    Brake,
    Hold,
}

impl FromStr for StopAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "coast" => StopAction::Coast,
            "brake" => StopAction::Brake,
            "hold" => StopAction::Hold,
            _ => bail!("provided string was not `coast`, `brake` or `hold`"),
        })
    }
}

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
