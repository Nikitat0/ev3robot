use derive_more::*;

macro_rules! unit {
    (
        $(#[$outer:meta])*
        pub struct $name:ident($(#[$inner:meta])* $repr:ty);
    ) => {
         $(#[$outer])*
         #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Constructor,
            From,
            Display,
            Binary,
            Octal,
            LowerHex,
            UpperHex,
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
    pub struct Revolutions(i32);
}
