// Arrays and tuples are fixed length/size and thus called compound types
// Rust has two primitive compound types: Arrays and tuples
// Compound types are stored on the stack

// Vectors, Strings and Hashmaps are collections
// Data these collections point to is stored on the heap

pub fn run() {

    // Array: Fixed list (size), all elements same type
    // - All stack allocated (thus no push/pop)
    let num_array: [i32; 4] = [1,2,3,4];

    let slice: &[i32] = &num_array[3..4];

    println!("{:?}", slice);


    // Tuple of &str slice and integer
    let tuple: (&str, i32) = ("A String", 1337);
    let x = tuple; 
    println!("{:?}", x);
        


    // Creating a vector with the macro "vec!"
    let mut v: Vec<u16> = vec![1,2,3,4,5];
    v.push(1337);
    println!("{:?}", v);




}