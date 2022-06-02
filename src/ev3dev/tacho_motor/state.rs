use std::str::FromStr;

use bitflags::bitflags;
use thiserror::Error;

bitflags! {
    #[derive(Default)]
    pub struct State: u8 {
        const RUNNING = 1;
        const RAMPING = 2;
        const HOLDING = 4;
        const OVERLOADED = 8;
        const STALLED = 16;
    }
}

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|flag| {
                Ok(match flag {
                    "running" => State::RUNNING,
                    "ramping" => State::RAMPING,
                    "holding" => State::HOLDING,
                    "overloaded" => State::OVERLOADED,
                    "stalled" => State::STALLED,
                    invalid_flag => {
                        return Err(ParseStateError {
                            invalid_flag: invalid_flag.to_string(),
                        })
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
#[error("invalid flag `{}`", invalid_flag)]
pub struct ParseStateError {
    invalid_flag: String,
}
