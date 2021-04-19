// Lifetimes 
#[allow(unused_variables)] 

pub fn run() {
    println!("module lifetimes: run() called \n---------");

    // Scope basic example
    let x;

    {       
        let y = 1; // Inner scope
        x = &y;
        println!("var x printed from inside scope: {}", x); 
    }
    // ERROR: y's lifetime is not long enough to exist outside of inner scope
    // println!("{}", x); 

    // ERROR: borrowed reference cannot originate from inside function
    // fn return_ref() -> &u8 {
    //     let i = 20;
    //     return &i;
    // }



    let int = 1338;
    let ref_from_fn = get_ref(&int);
    println!("{}", ref_from_fn);

}

// usually the compiler figures it out and lifetimes are being added implicitly 
// <'a> syntax is verbose/explicit
pub fn get_ref<'a>(param: &'a u16) -> &'a u16 {
    param
}


pub fn get_ref_2<'a, 'scnd>(param: &'a u16, param2: &'scnd usize) -> (&'a u16, &'scnd usize) {
    (param, param2)
}