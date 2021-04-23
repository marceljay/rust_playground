// Generics
// useful for reducing code duplication 

// Example of a function that is generic over some type T
// 1) the <T> is the declaration of the type
// 2) parameter T is a slice (thus &) of type T
// 3) returned is a reference to a value of type T
fn generic_function<T>(param: &[T]) -> &T {
    &param[0]
}

// Requires a restriction as binary operation is dependent on actual type
fn compare<T: std::cmp::PartialOrd>(param: &[T]) -> &T {
    
    let a = &param[0];
    let b = &param[1];
    // ERROR: cannot add &T to &T... why?
    // let res = a+b;
    if a > b {
        a
    }
    else {
        b
    }
}

#[allow(unused_variables)] 
pub fn run() {
    println!("\nmodule generics: run() called \n---------");


    
    let number_list = vec![34, 50, 25, 100, 65];

    let x = 12u32;

    let cond: bool = true;

    // using the <> syntax we can define fields with generic types
    struct Point<T> {
        x: T,
        y: T,
        z: u8,
    }
    
    let integer = Point { x: 5, y: 10, z: 1};
    let float = Point { x: 1.337, y: 4.0, z: 0 };

    // two different generic types in one struct
    struct Some<T,V> {
        var: T,
        msg: V,
    }
    
    let gen = Some { var: 1.337, msg: "hello".to_string() };
    println!("gen.msg: {}", gen.msg )


}