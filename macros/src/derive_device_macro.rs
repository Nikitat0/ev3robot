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
    data: darling::ast::Data<(), DeviceField>,
}

impl ToTokens for Device {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { ident, generics, class, data } = self;
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        let field_inits = data.clone().take_struct().unwrap().fields;
        tokens.extend(quote! {
            impl #impl_generics ev3robot::device::Device
                for #ident #ty_generics #where_clause
            {
                const CLASS: &'static str = #class;

                fn try_connect<S>(__device_name: S)
                    -> ev3robot::device::ConnectionResult<Self>
                where
                    S: ::std::convert::AsRef<::std::ffi::OsStr>
                {
                    Ok(Self {#(#field_inits),*})
                }
            }
        })
    }
}

#[derive(Clone, FromField)]
#[darling(attributes(ev3robot))]
struct DeviceField {
    ident: Option<syn::Ident>,
    #[darling(default)]
    name: Option<String>,
}

impl ToTokens for DeviceField {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, ident } = self;
        let name = name
            .as_ref()
            .map(Clone::clone)
            .or_else(|| ident.as_ref().map(ToString::to_string));
        tokens.extend(quote! {
            #ident: ev3robot::device::Attribute::of_device(
                Self::CLASS,
                &__device_name,
                #name,
            ).map_err(|err| {
                ev3robot::device::ConnectionError::new_unexpected_with_context(
                    err,
                    ::std::format!("Error in attribute {}", #name)
                )
            })?
        })
    }
}
