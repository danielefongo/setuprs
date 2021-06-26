use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Brace;
use syn::{braced, Ident, LitStr};

#[derive(Clone)]
pub struct DynamicBlock<T: Parse> {
    pub brace_token: Brace,
    pub items: Vec<T>,
}

impl<T: Parse> DynamicBlock<T> {
    pub fn append_on_start(&mut self, vect: Vec<T>) {
        vect.into_iter().rev().for_each(|item| {
            self.items.insert(0, item);
        });
    }
}

impl<T: Parse> Parse for DynamicBlock<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }
            Ok(DynamicBlock { brace_token, items })
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone)]
pub struct StringedIdent {
    pub kind: Ident,
    pub ident: Ident,
}

impl Parse for StringedIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let kind: Ident = input.parse()?;
        let description: LitStr = input.parse()?;
        let ident_string = kind.to_string() + "_" + &description.value().replace(" ", "_");
        let ident = Ident::new(&ident_string, Span::call_site());
        Ok(StringedIdent { kind, ident })
    }
}

impl ToTokens for StringedIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        &self.ident.to_tokens(tokens);
    }
}
