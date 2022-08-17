use std::str::FromStr;

use anyhow::bail;
use bitflags::bitflags;

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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        s.split_whitespace()
            .map(|flag| {
                Ok(match flag {
                    "running" => State::RUNNING,
                    "ramping" => State::RAMPING,
                    "holding" => State::HOLDING,
                    "overloaded" => State::OVERLOADED,
                    "stalled" => State::STALLED,
                    invalid_flag => {
                        bail!("invalid flag `{}`", invalid_flag)
                    }
                })
            })
            .collect()
    }
}
