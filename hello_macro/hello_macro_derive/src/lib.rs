extern crate proc_macro;  // comes with Rust

use proc_macro::TokenStream;
// uote crate turns syn data structures back into Rust code
use quote::quote;
// syn crate parses Rust code from a string into a data structure
use syn;

// LOGIC: 
// when #[derive(HelloMacro)] is specified on a type 
// Then hello_macro_derive(..) is called
// Because: #[proc_macro_derive(HelloMacroTrait)] was annotated


#[proc_macro_derive(HelloMacroTrait)]
// hello_macro_derive() is responsible for parsing the TokenStream, and the impl_hello_macro function, 
// impl_hello_macro() is responsible for transforming the syntax tree: 
// this makes writing a procedural macro more convenient
// the code in this outer function is the same for almost every proc macro

// returned TokenStream is added to the code that our crate users write when compiled

pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // TokenStream --> DeriveInput
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation

    impl_hello_macro(&ast)
}



fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // quote! macro lets us define the Rust code that we want to return
    let gen = quote! {
        impl HelloMacroTrait for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // into method consumes this intermediate representation and returns a value of the required TokenStream type.
    gen.into()
}


// DeriveInput {
//     // --snip--

//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }