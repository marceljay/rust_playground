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


    // Turbofish to fix problems

    let numbers: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    // Without the Turbofish, the type could not be inferred
    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        // Here it means “collect this iterator into a Vec<i32>”
        .collect::<Vec<i32>>();
        // which could also be written as
        // .collect::<Vec<_>>();
    

    println!("{:?}", even_numbers);
    
    
}


