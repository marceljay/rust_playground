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
    let int2 = 7;
    let ref_from_fn = get_ref(&int);
    println!("{}", ref_from_fn);
    get_ref_3(&int, &int2);

}

// usually the compiler figures it out and lifetimes are being added implicitly 
// <'a> syntax is verbose/explicit
pub fn get_ref<'a>(param: &'a u16) -> &'a u16 {
    param
}

// each param can have their own lifetime (but does not need to)
pub fn get_ref_2<'a, 'b>(param: &'a u16, param2: &'b usize) -> (&'a u16, &'b usize) {
    (param, param2)
}

// multiple lifetimes defined, not used, throws no error
pub fn get_ref_3<'a, 'b, 'c>(param: &'a u16, param2: &u32) -> u16 {
    println!("{} {}", param, param2); 
    return 1337
}


// Both params have the same lifetime
pub fn get_ref_4<'a>(param: &'a u16, param2: &'a u16) -> &'a u16 {
    if param > param2 {
        param
    }
    else {
        param2
    }
}