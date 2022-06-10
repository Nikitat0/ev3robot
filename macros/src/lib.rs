#[macro_use]
extern crate darling;

use proc_macro::TokenStream;

mod derive_device_macro;
mod derive_findable_device_macro;

#[proc_macro_derive(Device, attributes(ev3robot))]
pub fn derive_device(input: TokenStream) -> TokenStream {
    derive_device_macro::derive_device_impl(input.into()).into()
}

#[proc_macro_derive(FindableDevice, attributes(findable_device))]
pub fn derive_findable_device(input: TokenStream) -> TokenStream {
    derive_findable_device_macro::derive_findable_device_impl(input.into())
        .into()
}
