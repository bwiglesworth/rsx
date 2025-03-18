use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, Token, Ident, LitStr, Result, parse::ParseStream};

struct Element {
    tag: Ident,
    attributes: Vec<XmlAttribute>,
    children: Vec<Element>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let tag = input.parse()?;
        let attributes = parse_attributes(input)?;
        input.parse::<Token![>]>()?;
        
        let children = parse_children(input)?;
        
        // Parse closing tag separately
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let closing_tag = input.parse::<Ident>()?;
        if closing_tag != tag {
            return Err(input.error("mismatched tags"));
        }
        input.parse::<Token![>]>()?;
        
        Ok(Element {
            tag,
            attributes,
            children,
        })
    }
}


#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Element);
    
    let expanded = quote! {
        Component::with_props(Props::default())
    };
    
    TokenStream::from(expanded)
}

fn parse_attributes(input: ParseStream) -> syn::Result<Vec<XmlAttribute>> {
    let mut attrs = Vec::new();
    
    while !input.peek(Token![>]) {
        let name: syn::Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        
        attrs.push(XmlAttribute { name, value, equals: input.parse()? });
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