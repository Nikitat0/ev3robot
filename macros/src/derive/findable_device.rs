use darling::util::Flag;
use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

#[derive(FromDeriveInput)]
#[darling(attributes(findable_device))]
pub struct FindableDevice {
    ident: syn::Ident,
    generics: syn::Generics,
    class: String,
    driver: Option<String>,
    no_findable_impl: Flag,
    no_findable_by_port_impl: Flag,
}

impl FindableDevice {
    fn findable_device_impl(&self) -> TokenStream {
        let Self { ident, generics, class, driver, .. } = self;
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        let filter_by_driver: Option<syn::Stmt> =
            driver.as_ref().map(|driver| {
                parse_quote! {
                    let device_nodes =
                        device_nodes .filter(|node| {
                            crate::device::utils::device_node_driver_name(
                                node
                            ).map(|it| it == #driver).unwrap_or_default()
                        });
                }
            });
        quote! {
            impl #impl_generics crate::device::FindableDevice
                for #ident #ty_generics #where_clause
            {
                fn find_device_nodes() -> ::std::vec::Vec<::std::path::PathBuf>
                {
                    use ::std::iter::Iterator;
                    let device_nodes =
                        crate::device::utils::find_device_nodes_by_class(
                            #class
                        ).into_iter();
                    #filter_by_driver
                    device_nodes.collect()
                }
            }
        }
    }

    fn findable_impl(&self) -> Option<TokenStream> {
        let Self { ident, generics, no_findable_impl, .. } = self;
        if no_findable_impl.is_present() {
            return None;
        }
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        Some(quote! {
            impl #impl_generics crate::find::Findable
                for #ident #ty_generics #where_clause
            {
                fn find() -> crate::find::Result<Self> {
                    use crate::device::Device as _;
                    use crate::device::FindableDevice as _;
                    let device_node = crate::find::find_in(
                        Self::find_device_nodes()
                    )?;
                    Ok(Self::open(device_node)?)
                }
            }
        })
    }

    fn findable_by_port_impl(&self) -> Option<TokenStream> {
        let Self { ident, generics, no_findable_by_port_impl, .. } = self;
        if no_findable_by_port_impl.is_present() {
            return None;
        }
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();
        Some(quote! {
            impl #impl_generics crate::find::FindableBy<crate::port::Port>
                for #ident #ty_generics #where_clause
            {
                fn find_by<U>(port: U) -> crate::find::Result<Self>
                where
                    U: ::std::borrow::Borrow<crate::port::Port>
                {
                    use crate::device::Device as _;
                    use crate::device::FindableDevice as _;
                    use crate::device::utils::*;
                    let port = ::std::borrow::Borrow::borrow(&port);
                    let device_node = crate::find::find_in(
                        #ident::find_device_nodes()
                            .into_iter()
                            .filter(|device_node|{
                                device_node_port(device_node)
                                    .map(|it|{
                                        &it == port
                                    })
                                    .unwrap_or_default()
                            })
                    )?;
                    Ok(#ident::open(device_node)?)
                }
            }
        })
    }
}

impl ToTokens for FindableDevice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let findable_device_impl = self.findable_device_impl();
        let findable_impl = self.findable_impl();
        let findable_by_port_impl = self.findable_by_port_impl();
        tokens.extend(quote! {
            #findable_device_impl
            #findable_impl
            #findable_by_port_impl
        })
    }
}
