use proc_macro::TokenStream;
use quote::quote;
use serde_json::Value;
use std::fs;
use syn::{parse_macro_input, parse::Parse, parse::ParseStream, Token, LitStr, Expr};

struct MacroInput {
    json_path: LitStr,
    _comma: Token![,],
    func: Expr,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MacroInput {
            json_path: input.parse()?,
            _comma: input.parse()?,
            func: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn synth_cpu(input: TokenStream) -> TokenStream {
    let MacroInput { json_path, func, .. } = parse_macro_input!(input as MacroInput);
    
    // Read and parse the JSON file 
    let json_content = fs::read_to_string(json_path.value())
        .expect("Failed to read the JSON file");
    let json: Value = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");

    // Navigate to `cells` in the JSON structure
    let cells = json["modules"]["cpu"]["cells"]
        .as_object()
        .expect("Expected cells to be an object");

    // Generate statements for each cell
    let mut statements = Vec::new();
    for (_cell_name, cell_data) in cells {
        let cell_type = cell_data["type"].as_str().unwrap_or("unknown");
        let connections = cell_data["connections"].as_object().unwrap();

        assert!(cell_type == "NAND");

        let get = |key| {
            let arr = connections.get(key);
            assert!(arr.is_some());
            let arr = arr.unwrap();
            let arr = arr.as_array();
            assert!(arr.is_some());
            let arr = arr.unwrap();

            let val = arr[0].as_u64();

            let val = if val.is_some() {
                let nxt = val.unwrap();

                assert!(nxt != 0 && nxt != 1);

                nxt
            } else {
                let nxt = arr[0].as_str().unwrap().parse().unwrap();

                assert!(nxt == 0 || nxt == 1);

                nxt
            };

            usize::try_from(val).unwrap()
        };

        let input1 = get("A");
        let input2 = get("B"); 
        let output = get("Y");

        statements.push(quote! {
            #func(#input1, #input2, #output);
        });
    }

    // Generate the final token stream
    let output = quote! {
        {
            #(#statements)*
        }
    };

    output.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
