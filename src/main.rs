#![allow(dead_code)]  // crate level attribute, because of hash AND bang (!)

mod functions;
mod collections;

#[allow(unused_variables)] 

fn main() {

    collections::run();

    // Closures - access to variables outside function
    let a = 10;
    let closure = |n1: i8, n2: i8| a + n1 + n2 + a;
    println!("Closure {}", closure(1,2));

    // String slices (byte array) on stack 
    // By changing the string, a new slice on the stack is created (new address)
    let mut s: &str = "init";
    println!("{} Pointer: {:?}", s, s.as_ptr());

    let copy1 = s;          // Works because data is on the stack (Copy trait)
    s = "changed"; 
    println!("{} Pointer: {:?}", s, s.as_ptr());
    let copy2 = s;
    println!("{} Pointer: {:?}", copy2, copy2.as_ptr());
    s = "init"; 
    println!("{} Pointer: {:?}", s, s.as_ptr());
    // let mut typecheck: () = s;

    assert_ne!(4, s.len());     // no panic
    // assert_eq!(s, copy1);    // would panic
    
    println!("----------");

    // Moving ownership of a String struct on the Heap
    let mut some_string = String::from("I am some_string");

    // Adding some characters
    some_string.push_str(" and now a bit longer ");
    // let x: () = some_string;

    println!("some_string {}", some_string);
    let some_copy = some_string;
    println!("some_copy {}", some_copy);
    // let x: () = some_string;
    // Error, as old owner does have it in memory anymore
    // sprintln!("some_string {}", some_string);  



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


    // Tuple of &str slice and integer
    let tuple: (&str, i32) = ("A String", 1337);
    let x = tuple; 
    println!("{:?}", x);
    

    // For loops
    let end = 2;
    for x in 0..end {
        println!("Numba go up {}", x)
    }

    
// match x {
//     1 => println!("one"),
//     2 => println!("two"),
//     3 => println!("three"),
//     4 => println!("four"),
//     5 => println!("five"),
//     _ => println!("something else"),
// }

//     std::io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess);
// 

}