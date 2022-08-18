use std::fmt::{self, Debug, Formatter};
use std::str::FromStr;

use anyhow::anyhow;
use derive_more::*;

macro_rules! percentage {
    ($name:ident, $range:pat, $repr:ty) => {
        #[derive(
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
        )]
        pub struct $name($repr);

        impl $name {
            pub fn new(percent: $repr) -> anyhow::Result<Self> {
                percent.try_into()
            }

            pub fn to_fraction(self) -> f32 {
                self.0 as f32 / 100_f32
            }
        }

        impl TryFrom<$repr> for $name {
            type Error = anyhow::Error;

            fn try_from(value: $repr) -> anyhow::Result<Self> {
                if let $range = value {
                    Ok($name(value))
                } else {
                    Err(anyhow!("invalid percentage"))
                }
            }
        }

        impl FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> anyhow::Result<Self> {
                s.parse::<$repr>()?.try_into()
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}%", self.0)
            }
        }

        impl From<$name> for $repr {
            fn from($name(percent): $name) -> Self {
                percent
            }
        }
    };
}

percentage!(Percentage, 0..=100, u8);
percentage!(SignedPercentage, -100..=100, i8);
