// There are 3 kinds of procedural macros

// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument
// Custom #[derive] macros (as implemented here and in the crates)

use hello_macro::HelloMacroTrait;
use hello_macro_derive::HelloMacroTrait;

#[derive(HelloMacroTrait)]
struct Pancakes;



pub fn run() {
    println!("\nmodule proc_macros: run() called \n---------");
    Pancakes::hello_macro();
}