use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Json)]
pub fn json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = match input.data {
        Data::Struct(s) => {
            let fields = s.fields.into_iter().map(|field| field.ident.unwrap());
            quote! {
                impl json_trait::Json for #name {
                    fn to_json(&self) -> String {
                        let mut json = "{ ".to_string();
                        #(
                            json.push_str(&format!("\"{}\": {}, ", stringify!(#fields), json_trait::Json::to_json(&self.#fields)));
                        )*
                        json.remove(json.len() - 2); // remove trailling comma
                        json.push('}');
                        json
                    }
                }
            }
        },
        Data::Enum(e) => {
            let variants = e.variants.into_iter().map(|variant| variant.ident);
            quote! {
                impl json_trait::Json for #name {
                    fn to_json(&self) -> String {
                        match &self {
                            #(Self::#variants => format!("\"{}\"", stringify!(#variants)) ),*
                        }
                    }
                }
            }
        }
        _ => todo!(),
    };

    out.into()
}