#[macro_use]
extern crate darling;

use proc_macro::TokenStream;

mod derive;
use derive::derive;

macro_rules! pub_derive_macro {
    ($name:ident, $helper_attr:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name, attributes($helper_attr))]
        pub fn $name(input: TokenStream) -> TokenStream {
            derive::<derive::$name>(input)
        }
    };
}

pub_derive_macro!(Device, device);
pub_derive_macro!(FindableDevice, findable_device);
