use hello_macro::HelloMacroTrait;
use hello_macro_derive::HelloMacroTrait;

#[derive(HelloMacroTrait)]
struct Pancakes;



pub fn run() {
    println!("\nmodule proc_macros: run() called \n---------");
    Pancakes::hello_macro();
}