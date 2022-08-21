use derive_more::*;

macro_rules! unit {
    {
        $(#[$outer:meta])*
        pub struct $name:ident($repr:ty);
    } => {
        $(#[$outer])*
        #[derive(
            Clone, Copy, PartialEq, PartialOrd, Display, From,
        )]
        pub struct $name($repr);

        impl $name {
            pub fn new(value: $repr) -> $name {
                $name(value)
            }

            pub fn value(self) -> $repr {
                self.into()
            }
        }

        impl From<$name> for $repr {
            fn from($name(value): $name) -> Self {
                value
            }
        }
    };
}

unit! {
    #[derive(Debug, Eq, Ord, FromStr)]
    pub struct TachoCounts(i32);
}
