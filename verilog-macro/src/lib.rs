use proc_macro::TokenStream;
use quote::quote;
use serde_json::Value;
use std::fs;

#[proc_macro]
pub fn json_to_println(input: TokenStream) -> TokenStream {
    // Parse the input (JSON file path as a string literal)
    let input_str = input.to_string();
    let file_path = input_str.trim_matches('"');

    // Read and parse the JSON file
    let json_content = fs::read_to_string(file_path)
        .expect("Failed to read the JSON file");
    let json: Value = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");

    // Navigate to `cells` in the JSON structure
    let cells = json["modules"]["counter"]["cells"]
        .as_object()
        .expect("Expected cells to be an object");

    // Generate println! statements for each cell
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
            self.nand(#input1, #input2, #output)?;
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
