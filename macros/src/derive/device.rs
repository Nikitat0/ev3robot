use darling::FromDeriveInput;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_quote;

pub fn derive(raw_input: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2(raw_input) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    let device_struct = match DeviceStruct::from_derive_input(&input) {
        Ok(device_impl) => device_impl,
        Err(err) => return err.write_errors(),
    };
    let device_impl = device_struct.gen_impl();

    quote! { #device_impl }
}

#[derive(FromDeriveInput)]
#[darling(attributes(device), supports(struct_named))]
struct DeviceStruct {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), DeviceField>,
    #[darling(multiple, rename = "apply")]
    fns_to_apply: Vec<syn::Expr>,
}

impl DeviceStruct {
    fn gen_impl(&self) -> TokenStream2 {
        let Self { ident, generics, data, fns_to_apply } = self;
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        let fields_inits = data
            .as_ref()
            .take_struct()
            .unwrap()
            .into_iter()
            .map(DeviceField::gen_field_init);
        let applications = fns_to_apply.into_iter().map(|it| -> syn::Stmt {
            parse_quote! {#it(&mut device)?;}
        });

        quote! {
            impl #impl_generics crate::device::Device
                for #ident #ty_generics #where_clause
            {
                fn open<P>(device_node: P) -> crate::__anyhow::Result<Self>
                where
                    P: ::std::convert::AsRef<::std::path::Path>
                {
                    use crate::__anyhow::Context;
                    let device_node = device_node.as_ref();
                    let mut device = Self {#(#fields_inits),*};
                    #(#applications)*
                    Ok(device)
                }
            }
        }
    }
}

#[derive(FromField)]
#[darling(attributes(ev3robot))]
struct DeviceField {
    ident: Option<syn::Ident>,
    #[darling(default)]
    attr_name: Option<String>,
}

impl DeviceField {
    fn gen_field_init(&self) -> TokenStream2 {
        let ident = &self.ident;
        let init_expr = self.gen_field_init_expr();
        quote! {#ident: #init_expr}
    }

    fn gen_field_init_expr(&self) -> syn::Expr {
        let field_name = self.ident.as_ref().unwrap().to_string();
        let attr_name =
            self.attr_name.as_ref().map(String::as_str).unwrap_or(&field_name);
        parse_quote! {
            crate::device::Attribute::of_device(
                device_node,
                #attr_name,
            ).context(format!("Error in attribute {}", #attr_name))?
        }
    }
}
