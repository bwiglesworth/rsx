use syn::{parse::{Parse, ParseStream}, Token, Result};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
struct XmlNode {
    tag: syn::Ident,
    props: Vec<XmlProp>,
    children: Vec<XmlNode>,
}

#[derive(Debug)]
struct XmlProp {
    name: syn::Ident,
    value: syn::Expr,
}

impl Parse for XmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let tag = input.parse()?;
        syn::bracketed!(content in input);
        
        let props = content.parse_terminated(XmlProp::parse, Token![,])?;
        let children = if input.peek(Token![{]) {
            let content;
            syn::braced!(content in input);
            content.parse_terminated(XmlNode::parse, Token![,])?
        } else {
            Default::default()
        };

        Ok(XmlNode {
            tag,
            props: props.into_iter().collect(),
            children: children.into_iter().collect(),
        })
    }
}
