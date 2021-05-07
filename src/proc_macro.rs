use hello_macro::HelloMacroTrait;
use hello_macro_derive::HelloMacroTrait;

#[derive(HelloMacroTrait)]
struct Pancakes;



pub fn run() {
    Pancakes::hello_macro();
}