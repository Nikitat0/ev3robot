use darling::FromDeriveInput;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_quote;

pub fn derive(raw_input: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2(raw_input) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    let device_struct = match FindableDeviceStruct::from_derive_input(&input) {
        Ok(device_impl) => device_impl,
        Err(err) => return err.write_errors(),
    };
    let device_impl = device_struct.gen_impl();

    quote! { #device_impl }
}

#[derive(FromDeriveInput)]
#[darling(attributes(findable_device))]
struct FindableDeviceStruct {
    ident: syn::Ident,
    generics: syn::Generics,
    class: String,
    driver: Option<String>,
}

impl FindableDeviceStruct {
    fn gen_impl(&self) -> TokenStream2 {
        let Self { ident, generics, class, driver } = self;
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        let filter_by_driver: Option<syn::Stmt> =
            driver.as_ref().map(|driver| {
                parse_quote! {
                    let device_nodes =
                        device_nodes .filter(|node| {
                            ev3robot::device::utils::device_node_driver_name(
                                node
                            ).map(|it| it == #driver).unwrap_or_default()
                        });
                }
            });

        quote! {
            impl #impl_generics ev3robot::device::FindableDevice
                for #ident #ty_generics #where_clause
            {
                fn find_device_nodes() -> ::std::vec::Vec<::std::path::PathBuf>
                {
                    use ::std::iter::Iterator;
                    let device_nodes =
                        ev3robot::device::utils::find_device_nodes_by_class(
                            #class
                        ).into_iter();
                    #filter_by_driver
                    device_nodes.collect()
                }
            }
        }
    }
}
