#![allow(dead_code)]  // crate level attribute, because of hash (#) AND bang (!)

use std::io; // not in the prelude

mod functions;
mod collections;

// non-sensical function that returns two values. usize for needed because lenght unknown?
// Rust is pass-by-value, which is why the &ampersand is needed
fn greeter(name: &str) -> (&str, usize) {
    println!("Welcome to my rust playground");

    (name, name.len())
}

#[allow(unused_variables)] 
fn main() {

    println!("Hello {:?}", greeter("Marcel"));
    collections::run();

    // Closures - access to variables outside function
    let a = 10;
    let closure = |n1: i8, n2: i8| a + n1 + n2 + a;
    println!("Closure {}", closure(1,2));

  
    // let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;
    // println!("{:?} {:?}", r1, r2);


    // Permissions when borrowing
    let var_a = String::from("I'm a var");
    // let mut var_b = &var_a;
    // println!("\nvar_a {} \nvar_b {}\n", var_a, var_b);
    // var_b.push('A');
    // var_a = "fsdfsdfsdf";



    // For loops
    let end = 2;
    for x in 0..end {
        println!("Numba go up {}", x)
    }

    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");
    println!("Your selection {:?} ", choice);


    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     4 => println!("four"),
    //     5 => println!("five"),
    //     _ => println!("something else"),
    // }



}