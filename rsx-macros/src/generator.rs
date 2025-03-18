use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for XmlNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag = &self.tag;
        let props = self.props.iter().map(|prop| {
            let name = &prop.name;
            let value = &prop.value;
            quote! { #name: #value }
        });
        
        let children = &self.children;
        
        tokens.extend(quote! {
            ComponentBuilder::new(#tag::new())
                .with_props(#tag::Props {
                    #(#props,)*
                })
                .with_children(vec![#(#children),*])
                .build()
        });
    }
}
