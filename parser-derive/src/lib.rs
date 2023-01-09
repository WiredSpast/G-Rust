use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

#[proc_macro_derive(BaseParser)]
pub fn base_parser_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl BaseParser for #name {
            fn parse(packet: &mut HPacket) -> Self {
                packet.read()
            }

            fn append_to_packet(&self, packet: &mut HPacket) {
                packet.append(self.clone());
            }
        }
    };

    TokenStream::from(expanded)
}