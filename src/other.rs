pub fn run() {
    println!("\nmodule other: run() called");

    // For loops
    let end = 2;
    for x in 0..end {
        println!("Numba go up {}", x)
    }


    // Closures - access to variables outside function
    let a = 10;
    let closure = |n1: i8, n2: i8| a + n1 + n2 + a;
    println!("Closure {}", closure(1,2));
}