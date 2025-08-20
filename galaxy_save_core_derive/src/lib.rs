use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;

mod receiver;

use receiver::HeaderSerializerInput;

/// The derive macro for generating an implementation of the `HeaderSerializer` trait.
#[proc_macro_derive(HeaderSerializer, attributes(header_serializer))]
pub fn derive_header_serializer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);

    expand_header_serializer_impl(input).into()
}

/// Expands the generated implementation of the `HeaderSerializer` trait into a token tree.
fn expand_header_serializer_impl(input: syn::DeriveInput) -> TokenStream {
    let receiver = HeaderSerializerInput::from_derive_input(&input).unwrap();
    let ident = receiver.ident();
    let attr_set = receiver.attr_token_stream();
    let attr_set_num = attr_set.len();
    let data_size = receiver.data_size_token_stream();

    quote! {
        impl HeaderSerializer for #ident {
            fn header_serializer() -> galaxy_save_core::bin::BinaryDataContentHeaderSerializer<#ident> {
                galaxy_save_core::bin::BinaryDataContentHeaderSerializer::from(vec![#(#attr_set),*])
            }

            fn header_size() -> usize {
                size_of::<u16>()
                    + size_of::<u16>()
                    + #attr_set_num
                    * size_of::<galaxy_save_core::bin::BinaryDataContentAttribute>()
            }

            fn data_size() -> usize {
                #(#data_size)+*
            }
        }
    }
}
