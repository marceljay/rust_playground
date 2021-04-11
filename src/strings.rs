#[allow(unused_variables)] 
pub fn run() {
    println!("module strings: run() called");
    // String slices (byte array) on stack 
    // By changing the string, a new slice on the stack is created (new address)
    let mut s: &str = "init";
    println!("Pointer: {:?} for s:{:?} ", s.as_ptr(), s);
    s = "changed"; 
    println!("Pointer: {:?} for s:{:?} ", s.as_ptr(), s);

    // "Moving"/Copying works because data is on the stack (Copy trait)
    let copy2 = s; 
    // Address still the same
    println!("Pointer: {:?} for copy2:{:?}", copy2.as_ptr(), copy2);

    // After changing the string back, pointer again the same
    s = "init"; 
    println!("Pointer: {:?} for s:{:?} ", s.as_ptr(), s);
    // let mut typecheck: () = s;

    assert_ne!(3, s.len());     // no panic
    // assert_eq!(s, copy1);    // would panic
    println!("----------");




    // Moving ownership of a String struct on the Heap
    let mut some_string = String::from("I am some_string");

    // Adding some characters
    some_string.push_str(" and now a bit longer ");
    // let x: () = some_string;

    println!("some_string {}", some_string);
    let some_copy = &some_string;
    println!("some_copy {}", some_copy);
    // let x: () = some_string;
    // Error, as old owner does have it in memory anymore
    println!("some_string {}", some_string);  

    
}