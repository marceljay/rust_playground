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

// using the <> syntax we can define fields with generic types
struct Point<T> {
    x: T,
    y: T,
    z: u8,
}

// Implementation of methods on the type Point<T>
// T must be declared right after impl keyword
impl<T> Point<T> {
    fn values(&self) -> (&T, u8) {
        (&self.x, self.z)
    }
}

// Implementation for type Point<f32> (other types will not have these)
// Does not require <T> declaration
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(unused_variables)] 
pub fn run() {
    println!("\nmodule generics: run() called \n---------");
    
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{:?}", compare(&number_list));
    let char_list = vec!['a', 'c', 'x', 'q'];
    println!("{:?}", compare(&char_list));
    let bool_list = vec![true, false, true, true];
    println!("{:?}", compare(&bool_list));
    let string_list = vec!["ahello", "canIdoThis"];
    println!("{:?}", compare(&string_list));

    // Number literal with type suffix
    let x = 12u32;
 
    let p = Point { x: 5, y: 10, z: 1};
    let float = Point { x: 1.337, y: 4.0, z: 0 };

    // Calls
    println!("x and z = {:?}", p.values());

    // ERROR: Instance of Point<T> is not f32, so it does not have this method
    // println!("x and z = {:?}", p.distance_from_origin());

    println!("x and z = {:?}", float.distance_from_origin());


    // two different generic types in one struct
    struct Some<T,V> {
        var: T,
        msg: V,
    }
    
    let gen = Some { var: 1.337, msg: "hello".to_string() };
    println!("gen.msg: {}", gen.msg );

    let integer = Some(5);
    println!("Some {:?}", integer );


}