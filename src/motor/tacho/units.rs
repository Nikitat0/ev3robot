use std::fmt::{Debug, Formatter};

use derive_more::*;

macro_rules! unit {
    (
        $(#[$outer:meta])*
        pub struct $name:ident(i32);
    ) => {
        unit!{
            BREAK_RECURSION
            $(#[$outer])*
            #[derive(Eq, Ord, Binary, Octal, LowerHex, UpperHex)]
            pub struct $name(i32);
        }

        impl $name {
            pub fn as_i32(&self) -> i32 {
                self.0
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub struct $name:ident(f32);
    ) => {
        unit!{
            BREAK_RECURSION
            $(#[$outer])*
            pub struct $name(f32);
        }

        impl $name {
            pub fn as_f32(&self) -> f32 {
                self.0
            }
        }
    };
    (
        $(BREAK_RECURSION)?
        $(#[$outer:meta])*
        pub struct $name:ident($inner:ty);
    ) => {
         $(#[$outer])*
         #[derive(
            Clone,
            Copy,
            PartialEq,
            PartialOrd,
            Constructor,
            From,
            Display,
            LowerExp,
            UpperExp,
        )]
        pub struct $name($inner);

        impl From<$name> for $inner {
            fn from(tacho_counts: $name) -> Self {
                tacho_counts.0
            }
        }
    };
}

unit! {
    #[derive(Debug)]
    pub struct TachoCounts(i32);
}

unit! {
    pub struct Degrees(i32);
}

impl Debug for Degrees {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°", self.0)
    }
}

unit! {
    pub struct Revolutions(f32);
}

impl Debug for Revolutions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rev", self.0)
    }
}

const DEGREES_PER_REVOLUTION: f32 = 360.0;

impl From<Degrees> for Revolutions {
    fn from(degrees: Degrees) -> Self {
        Revolutions::from(degrees.as_i32() as f32 / DEGREES_PER_REVOLUTION)
    }
}

impl From<Revolutions> for Degrees {
    fn from(revolutions: Revolutions) -> Self {
        Degrees::from((revolutions.as_f32() * DEGREES_PER_REVOLUTION) as i32)
    }
}
