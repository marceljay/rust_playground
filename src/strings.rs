#[allow(unused_variables)] 
pub fn strings() {

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
    
}