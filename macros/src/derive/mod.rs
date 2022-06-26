mod device;
mod findable_device;

use darling::FromDeriveInput;
pub use device::*;
pub use findable_device::*;
use proc_macro::TokenStream;
use quote::ToTokens;

pub fn derive<D>(raw_input: TokenStream) -> TokenStream
where
    D: FromDeriveInput + ToTokens,
{
    let input = match syn::parse(raw_input) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error().into(),
    };
    let derived = match D::from_derive_input(&input) {
        Ok(device_impl) => device_impl,
        Err(err) => return err.write_errors().into(),
    };
    derived.into_token_stream().into()
}
