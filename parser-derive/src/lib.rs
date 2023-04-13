use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(to))]
struct Opts {
    direction: Option<u8>
}

#[proc_macro_derive(BaseParser, attributes(to))]
pub fn base_parser_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let direction = opts.direction.expect("Missing direction");
    let name = input.ident;
    let name_string = name.to_string();

    let expanded = quote! {
        impl BaseParser for #name {
            fn parse(packet: &mut HPacket) -> Self {
                packet.read()
            }

            fn append_to_packet(&self, packet: &mut HPacket) {
                packet.append(self.clone());
            }

            fn get_direction() -> HDirection {
                if #direction == 0 { HDirection::ToClient } else { HDirection::ToServer }
            }

            fn get_packet_name() -> String {
                #name_string.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}