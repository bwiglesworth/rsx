use proc_macro::TokenStream;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    // Transform <Card className="my-card"> into Component instantiation
    let component = parse_component(input);
    quote! {
        Card {
            props: CardProps {
                class_name: "my-card".to_string(),
            }
        }
    }.into()
}
