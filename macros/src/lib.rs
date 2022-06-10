#[macro_use]
extern crate darling;

use proc_macro::TokenStream;

mod derive;

macro_rules! pub_derive_macro {
    ($module_name:ident as $macro_name:ident) => {
        #[proc_macro_derive($macro_name, attributes($module_name))]
        pub fn $module_name(input: TokenStream) -> TokenStream {
            derive::$module_name::derive(input.into()).into()
        }
    };
}

pub_derive_macro!(device as Device);
pub_derive_macro!(findable_device as FindableDevice);
