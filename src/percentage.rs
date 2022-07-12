use std::fmt::{Debug, Formatter};
use std::str::FromStr;

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

        impl TryFrom<$repr> for $name {
            type Error = anyhow::Error;

            fn try_from(value: $repr) -> Result<Self, Self::Error> {
                if let 0..=100 = value {
                    Ok($name(value))
                } else {
                    anyhow::bail!("invalid percentage")
                }
            }
        }

        impl FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<$repr>()?.try_into()
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}%", self.0)
            }
        }

        impl From<$name> for $repr {
            fn from($name(percent): $name) -> Self {
                percent
            }
        }

        impl From<$name> for f32 {
            fn from($name(percent): $name) -> Self {
                percent as f32 / 100_f32
            }
        }
    };
}

percentage!(Percentage, 0..=100, u8);
percentage!(SignedPercentage, -100..=100, i8);
