// Macros
// can be used for variadic functions (like println!())
// rust is an expression-based language

// General Syntax for custom macros
macro_rules! macro_name {
    () => {
        // What it does, e.g
        println!("Macros are cool.")
    };
}

#[allow(unused_macros)] 
macro_rules! expr_macro {
        
        (x => $e:expr) => {println!("I can take expressions as argument {}", $e);};
        (y => $e:expr) => {println!("I can take expressions as argument {}", $e);};

}


#[allow(unused_variables)] 
pub fn run() {
    println!("\nmodule macros: run() called \n---------");
    macro_name!();
    expr_macro!(x => 1 + 1);
    expr_macro!(y => if 0 < 1 {true} else {false});

    // ERROR: no rules for token z
    // expr_macro!(z => 1 + 1);

}