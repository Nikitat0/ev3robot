use std::str::FromStr;

use anyhow::anyhow;
use derive_more::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Display,
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
    AsRef,
)]
pub struct DutyCycle(i8);

impl DutyCycle {
    pub fn new(value: i8) -> anyhow::Result<Self> {
        value.try_into()
    }

    pub fn map<F>(self, f: F) -> anyhow::Result<Self>
    where
        F: FnOnce(i8) -> i8,
    {
        f(self.unwrap()).try_into()
    }

    pub fn unwrap(self) -> i8 {
        self.0
    }
}

impl TryFrom<i8> for DutyCycle {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        (-100..=100).contains(&value).then(|| Self(value)).ok_or_else(|| {
            anyhow!("out of range integral type conversion attempted")
        })
    }
}

impl FromStr for DutyCycle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;
        (-100..=100)
            .contains(&value)
            .then(|| Self(value))
            .ok_or_else(|| anyhow!("number too large to fit in target type"))
    }
}

impl From<DutyCycle> for i8 {
    fn from(duty_cycle: DutyCycle) -> Self {
        duty_cycle.unwrap()
    }
}
