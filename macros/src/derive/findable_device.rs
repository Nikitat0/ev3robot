use std::ops::Not;

use darling::util::Flag;
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
    no_findable_impl: Flag,
    no_findable_by_port_impl: Flag,
}

impl FindableDeviceStruct {
    fn gen_impl(&self) -> TokenStream2 {
        let Self {
            ident,
            generics,
            class,
            driver,
            no_findable_impl,
            no_findable_by_port_impl,
        } = self;
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

        let findable_impl = no_findable_impl.is_present().not().then(|| {
            quote! {
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
            }
        });

        let findable_by_port_impl =
            no_findable_by_port_impl.is_present().not().then(|| {
                quote! {
                    impl #impl_generics
                        crate::find::FindableBy<crate::port::Port>
                        for #ident #ty_generics #where_clause
                    {
                        fn find_by<U>(value: U) -> crate::find::Result<Self>
                        where
                            U: ::std::borrow::Borrow<crate::port::Port>
                        {
                            fn find_by_port(
                                port: &crate::port::Port
                            ) -> crate::find::Result<#ident> {
                                use crate::device::Device as _;
                                use crate::device::FindableDevice as _;
                                use crate::device::utils::*;
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
                            find_by_port(::std::borrow::Borrow::borrow(&value))
                        }
                    }
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

            #findable_impl
            #findable_by_port_impl
        }
    }
}
