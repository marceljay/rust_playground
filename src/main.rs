#![allow(dead_code)]  // CRATE level attribute, because of hash (#) AND bang (!)

// use std::io; // not in the prelude

// Modules section
// Example: 
// rust will expect to find either a my_module.rs file, or a my_module/mod.rs
mod my_module;
mod structs;
mod collections;
mod strings;
mod ownership;
mod enums;
mod lifetimes;
mod generics;
mod macros;
mod proc_macro;
mod other;
mod enum_option;
mod error_handling;


// non-sensical function that returns two values. 
// string slice and usize needed because lenght unknown?
// Rust is pass-by-value, which is why the &ampersand is needed. 
fn greeter(name: &str) -> (&str, usize) {
    println!("Welcome to my rust playground");
    (name, name.len())
}

#[allow(unused_variables)]  // following scope level attribute
fn main() {

    println!("Hello {:?}", greeter("Marcel"));
    // collections::run();
    structs::run();
    // strings::run();
    // collections::run();
    // ownership::run();
    enums::run();
    enum_option::run();
    // lifetimes::run();
    // generics::run();
    // macros::run();
    // proc_macro::run();
    // other::run();
    error_handling::run();
   

    // let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;
    // println!("{:?} {:?}", r1, r2);


    // Permissions when borrowing
    // let var_a = String::from("I'm a var");
    // let mut var_b = &var_a;
    // println!("\nvar_a {} \nvar_b {}\n", var_a, var_b);
    // var_b.push('A');
    // var_a = "fsdfsdfsdf";


    // let mut choice = String::new();

    // io::stdin()
    // .read_line(&mut choice)
    // .expect("Failed to read line");
    // println!("Your selection {:?} ", choice);


    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     4 => println!("four"),
    //     5 => println!("five"),
    //     _ => println!("something else"),
    // }



}