extern crate proc_macro;

use std::collections::HashMap;

// Step 1: Define the Fact trait
pub trait Fact {
    fn key(&self) -> &str;
}


// Step 2: Implement the Fact trait for custom types using a custom derive macro
pub mod fact_impl {
    use proc_macro::TokenStream;
    use super::Fact;

    // Define the custom derive macro
    use quote::quote;
    use syn::{parse_macro_input, DeriveInput};

    #[proc_macro_derive(Fact)]
    pub fn fact_derive(input: TokenStream) -> TokenStream {
        // Parse the input tokens into a syntax tree
        let input = parse_macro_input!(input as DeriveInput);
        // Get the identifier of the type being derived for
        let ident = &input.ident;

        // Generate the implementation of the Fact trait
        let expanded = quote! {
            impl Fact for #ident {
                fn description(&self) -> &str {
                    stringify!(#ident)
                }
            }
        };

        // Return the generated implementation as tokens
        TokenStream::from(expanded)
    }
}

// Step 3: Use a HashMap with trait objects to store values of different types that implement the Fact trait
// fn main() {
//     // Create a HashMap to store values of different types that implement the Fact trait
//     let mut fact_map: HashMap<&str, Box<dyn Fact>> = HashMap::new();
//
//     // Insert values into the HashMap
//     let string_fact = StringFact { value: "Hello".to_string() };
//     let int32_fact = Int32Fact { value: 42 };
//
//     fact_map.insert(string_fact.description(), Box::new(string_fact));
//     fact_map.insert(int32_fact.description(), Box::new(int32_fact));
//
//     // Access values from the HashMap
//     for (_, fact) in &fact_map {
//         println!("Description: {}", fact.key());
//     }
// }