use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{parse_macro_input, Ident, Token};
use syn::parse::{Parse, ParseStream};

struct Identifiers {
    idents: Vec<Ident>,
}

impl Parse for Identifiers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut idents = Vec::new();
        while !input.is_empty() {
            idents.push(input.parse()?);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Identifiers { idents })
    }
}

#[proc_macro]
pub fn concat_identifiers(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Identifiers);
    let idents = input.idents.iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>();

    let ident = Ident::new(&idents.join(""), Span::call_site());
    TokenStream::from(quote! {
        #ident
    })
}

