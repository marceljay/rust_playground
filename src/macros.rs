#![allow(unused_macros)] 
use proc_macro;

// Macros
// Metaprogramming: macros are a way of writing code that writes other code
// can be used for variadic functions (like println!())
// rust is an expression-based language


// General Syntax for "declarative macros" created via keyword "macro_rules!"
macro_rules! macro_name {
    () => {
        // What it does, e.g
        println!("Macros are cool.")
    };
}
 
#[macro_export] // makes it available whenever the crate is brought into scope
macro_rules! expr_macro {
        // Syntax similar to a Rust match expression, this macro has multiple arms
        // macro_rules!-Macros compare a value (literal) to patterns associated with code
        // <pattern> => <associated code to be emitted in case of match>
        (x => $e:expr) => {println!("I can take expressions as argument {}", $e);};
        (y => $e2:  expr) => {println!("I can take expressions as argument {}", $e2);};
        (token q(-_-)p $e:expr) => {println!("first part can be anything {:?}", $e);};
        ( $e:expr) => {println!("I can take expressions as argument {}", $e);};

        ($st:stmt) => {$st};

        // $e:expr matches any expression, $e is the expression's name
}


use std::collections::HashMap;


macro_rules! new_hashmap {

    // The asterisk * indicates that the value passed could repeat 
    // the $() is required to wrap the part for "copying"
    // no comma would mean SPACE is the delimiter after token
    // + sign instead of * requires expression to appear at least once
    ($($key:expr => $val:expr),*) => {

        {
            let mut map = HashMap::new();
            
            $(
                map.insert($key, $val);
            )*
            
         map
        
        }
    };

} 

// Procedural Macros (sort of like a mapping on input code)
// All procedural macros work in a similar fashion.
// The types are: custom derive, attribute-like, and function-like
// Example: 
// #[some_attribute]



#[allow(unused_variables)] 
pub fn run() {
    println!("\nmodule macros: run() called \n---------");
    macro_name!();
    expr_macro![x => 1 + 1]; // Note that the delimiter can be different
    expr_macro!{x => 1 + 1}; 

    expr_macro!(y => if 0 < 1 {let b = true; b} else {let b = false; b});

    // ERROR: no rules for token z
    // expr_macro!(z => 1 + 1);
    expr_macro!(token q(-_-)p false);
    expr_macro!(let numba: u32 = 5);

    let m_str = new_hashmap!["one" => 1, "two" => 2];
    let m_hex = new_hashmap![0x12 => 12, 0xab => 23];


}