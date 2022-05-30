use darling::FromDeriveInput;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

pub fn derive_device_impl(raw_input: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2(raw_input) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    let device_impl = match Device::from_derive_input(&input) {
        Ok(device_impl) => device_impl,
        Err(err) => return err.write_errors(),
    };

    quote! { #device_impl }
}

#[derive(FromDeriveInput)]
#[darling(attributes(ev3robot), supports(struct_named))]
struct Device {
    ident: syn::Ident,
    generics: syn::Generics,
    class: String,
}

impl ToTokens for Device {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { ident, generics, class } = self;
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        tokens.extend(quote! {
            impl #impl_generics ev3robot::device::Device
                for #ident #ty_generics #where_clause
            {
                const CLASS: &'static str = #class;

                fn try_connect<S>(name: S)
                    -> ev3robot::device::ConnectionResult<Self>
                where
                    S: ::std::convert::AsRef<::std::ffi::OsStr>
                {
                    todo!()
                }
            }
        })
    }
}
