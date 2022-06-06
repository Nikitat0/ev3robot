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
    data: darling::ast::Data<(), DeviceField>,
}

impl ToTokens for Device {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { ident, generics, data } = self;
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        let field_inits = data.clone().take_struct().unwrap().fields;
        tokens.extend(quote! {
            impl #impl_generics ev3robot::device::Device
                for #ident #ty_generics #where_clause
            {
                fn open<P>(device_node: P) -> ev3robot::__anyhow::Result<Self>
                where
                    P: ::std::convert::AsRef<::std::path::Path>
                {
                    use ev3robot::__anyhow::Context;
                    let device_node = device_node.as_ref();
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
                device_node,
                #name,
            ).context(format!("Error in attribute {}", #name))?
        })
    }
}
