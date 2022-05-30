#[macro_use]
extern crate darling;

use proc_macro::TokenStream;

mod derive_device_macro;

#[proc_macro_derive(Device, attributes(ev3robot))]
pub fn derive_device(input: TokenStream) -> TokenStream {
    derive_device_macro::derive_device_impl(input.into()).into()
}
