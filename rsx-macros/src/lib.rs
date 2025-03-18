use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, Token};

#[derive(Debug)]
struct Element {
    tag: syn::Ident,
    attrs: Vec<Attribute>,
    children: Vec<Element>,
}

#[derive(Debug)]
struct Attribute {
    name: syn::Ident,
    value: syn::Expr,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag = input.parse()?;
        let attrs = parse_attributes(input)?;
        let children = parse_children(input)?;
        
        Ok(Element { tag, attrs, children })
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let element = parse_macro_input!(input as Element);
    
    quote! {
        ComponentBuilder::new(#element.tag::new())
            .with_props(Props {
                #(#element.attrs)*
            })
            .with_children(vec![#(#element.children)*])
            .build()
    }.into()
}

fn parse_attributes(input: ParseStream) -> syn::Result<Vec<Attribute>> {
    let mut attrs = Vec::new();
    
    while !input.peek(Token![>]) {
        let name: syn::Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        
        attrs.push(Attribute { name, value });
    }
    
    Ok(attrs)
}

fn parse_children(input: ParseStream) -> syn::Result<Vec<Element>> {
    let mut children = Vec::new();
    
    input.parse::<Token![>]>()?;
    
    while !input.peek(Token![</]) && !input.is_empty() {
        children.push(input.parse()?);
    }
    
    input.parse::<Token![</]>()?;
    input.parse::<syn::Ident>()?;
    input.parse::<Token![>]>()?;
    
    Ok(children)
}