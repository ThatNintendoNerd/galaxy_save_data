use darling::{FromDeriveInput, FromField, ToTokens, ast};
use proc_macro2::TokenStream;
use quote::quote;

/// The abstract syntax tree for a `HeaderSerializer` implementor.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(header_serializer), supports(struct_named))]
pub struct HeaderSerializerInput {
    /// The name of the deriving type.
    ident: syn::Ident,

    /// The body of the deriving type.
    data: ast::Data<(), HeaderSerializerField>,
}

impl HeaderSerializerInput {
    /// Returns a reference to the name of the deriving type.
    pub fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    /// Creates a collection of token trees containing field descriptor expressions.
    pub fn attr_token_stream(&self) -> Vec<TokenStream> {
        let ast::Data::Struct(ref fields) = self.data else {
            panic!("receiver type should be a struct containing named fields");
        };

        let mut offset = Vec::with_capacity(fields.len());
        offset.push(quote! { 0 });

        fields
            .iter()
            .filter(|f| !f.skip)
            .map(|f| f.attr_token_stream(&mut offset))
            .collect()
    }

    /// Creates a collection of token trees containing type size expressions.
    pub fn data_size_token_stream(&self) -> Vec<TokenStream> {
        let ast::Data::Struct(ref fields) = self.data else {
            panic!("receiver type should be a struct containing named fields");
        };

        fields
            .iter()
            .filter(|f| !f.skip)
            .map(|f| f.data_size_token_stream())
            .collect()
    }
}

/// The abstract syntax tree for a field and its attributes.
#[derive(Debug, FromField)]
#[darling(attributes(header_serializer))]
struct HeaderSerializerField {
    /// The data type of the field.
    ty: syn::Type,

    /// The serialized name of the field.
    name: Option<String>,

    /// Determines if the field should not be considered.
    #[darling(default)]
    skip: bool,
}

impl HeaderSerializerField {
    /// Creates a token tree containing the field descriptor expression.
    fn attr_token_stream(&self, offset: &mut Vec<TokenStream>) -> TokenStream {
        let key = self.name.as_ref().unwrap();
        let quote = quote! {
            galaxy_save_core::bin::BinaryDataContentAttribute {
                key: galaxy_save_core::hash::HashCode::from(#key).into_raw() as u16,
                offset: #(#offset)+*
            }
        };

        let ty = self.ty();
        let offset_quote = quote! { size_of::<#ty>() as u16 };

        offset.push(offset_quote);

        quote
    }

    /// Creates a token tree containing the type size expression.
    fn data_size_token_stream(&self) -> TokenStream {
        let ty = self.ty();

        quote! {
            size_of::<#ty>()
        }
    }

    /// Returns the data type of the field as a token tree.
    fn ty(&self) -> TokenStream {
        match &self.ty {
            syn::Type::Array(ty) => ty.to_token_stream(),
            syn::Type::Path(ty) => ty.to_token_stream(),
            _ => panic!("field type should be a primitive"),
        }
    }
}
