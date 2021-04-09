pub fn run() {

    // Fixed list (size), all elements same type
    // - All stack allocated (thus no push/pop)
    let num_array: [i32; 4] = [1,2,3,4];

    let slice: &[i32] = &num_array[3..4];

    println!("{:?}", slice);


    // Creating a vector with the macro "vec!"
    let mut v: Vec<u16> = vec![1,2,3,4,5];
    v.push(1337);
    println!("{:?}", v)

}