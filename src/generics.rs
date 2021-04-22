// Generics
// useful for reducing code duplication 


#[allow(unused_variables)] 
pub fn run() {
    println!("\nmodule generics: run() called \n---------");

    // Example of a function that is generic over some type T
    // 1) the <T> is the declaration of the type
    // 2) parameter T is a slice (thus &) of type T
    // 3) returned is a reference to a value of type T
    fn generic_function<T>(param: &[T]) -> &T {
        &param[0]
    }
    
}