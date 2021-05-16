#![allow(unused_variables)] 

// Other languages have NULL, or the null pointer
// Rust does not have nulls, but it does have an enum 
// that can encode the concept of a value being present or absent.

// This was a deliberate design decision for Rust to limit null’s pervasiveness  
// increase the safety of Rust code.

// Everywhere a value has a type that isn’t an Option<T>, 
// you can safely assume that the value isn’t null. 

// standard library’s Option<T> enum is already included in the prelude
// (re)Defining it like this, would still be different from the original implementation
// certain methods like .unwrap() would not be available
enum Option<T> {
    Some(T),
    None,
}


pub fn run() {
    println!("\nmodule enum_option: run() called \n---------");

    // works because Some is included via standard library
    let optional_float = Some(1337);


    let some_number = Option2::Some(5);
    let some_string = Option2::Some("a string");
    
    // ERROR: Because generic enum variant could be dependent

    // let absent_number = Option2::JustNothing;


    // ERROR: compiler cannot infer type, and does not know how much memory to alloc 
    // let none: Option<i32> = None;
    // Thus: Binding `None` to a variable needs to be type annotated
    let _equivalent_none = None::<i32>;


    let optional_float = Some(1.337f32);
    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    let optional_float = Some(0f32);


}



enum Option2<T> {
    Some(T),
    JustNothing,
}
