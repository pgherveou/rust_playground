extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Token};

struct Feature {
    enabled: syn::Expr,
    disabled: syn::Expr,
}

impl Parse for Feature {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let enabled: syn::Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let disabled: syn::Expr = input.parse()?;
        Ok(Feature { enabled, disabled })
    }
}

#[allow(unused_variables)]
#[proc_macro]
pub fn my_feature(input: TokenStream) -> TokenStream {
    let Feature { enabled, disabled } = parse_macro_input!(input);

    #[cfg(feature = "my_feature")]
    let expanded = quote! {
        #enabled
    };

    #[cfg(not(feature = "my_feature"))]
    let expanded = quote! {
        #disabled
    };

    TokenStream::from(expanded)
}
