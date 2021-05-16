// Rust doesn’t have exceptions. 
// Instead, it has the type Result<T, E> for recoverable errors 
// and the panic! macro that stops execution when the program 
// encounters an unrecoverable error, e.g. a bug/erroneous code

//  When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

use std::env;

pub fn run() {
    println!("\nmodule error_handling: run() called \n---------");
    // panic!("crash and burn");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    // Rust only supports indexing of arrays/slices/vec using usize,
    let arg = args[1].parse::<usize>().unwrap();
    println!("arg: {:?}", arg);

    let v = vec![1, 2, 3];

    println!("lel {}", v[arg]);
}

