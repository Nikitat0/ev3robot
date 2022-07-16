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
        pub struct $name:ident($repr:ty);
    ) => {
         $(#[$outer])*
         #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            PartialOrd,
            Constructor,
            From,
            Display,
            LowerExp,
            UpperExp,
        )]
        pub struct $name($repr);

        impl From<$name> for $repr {
            fn from(tacho_counts: $name) -> Self {
                tacho_counts.0
            }
        }
    };
}

unit! {
    pub struct TachoCounts(i32);
}
unit! {
    pub struct Degrees(i32);
}
unit! {
    pub struct Revolutions(f32);
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
