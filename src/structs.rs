// Structs can be used to create custom data types
// There are regular structs and tuple structs with unnamed fields

// must be located in a sub-folder
mod other_struct;
use other_struct::NameTrait;
use other_struct::*; // allows me to omit 'namespace::struct'

mod other_trait;
use other_trait::*; // without use trait is not in scope

// Regular Struct
struct Person {
    age: u8,
    name: String
    // name: &str, // needs lifetime?
}

// A composed Struct that takes fields from another
struct PersonComposed {
    age: u8,
    name: String,
    // Composition, not inheritance
    composed: Accessible,
}

// Tuple Struct - unnamed fields
struct Numbers(u8, u16, usize);

// A unit struct
struct Unit;


// impl keyword is primarily used to define implementations on types
// e.g. Methods, member functions of a Struct
// According to Rust book:
// "their first parameter is always self, which represents the instance of the struct the method is being called on"
impl Person {

    // Works with or without ampersand, but without means value is consumed.
    fn get_age(&self) -> u8 {
        self.age
    }

    // Requires ampersand, because instance changes
    fn change_age(&mut self, new_age: u8) {
        self.age = new_age;
    }

    // Requires ampersand even though nothing changed
    fn get_name(&self) -> String {
        // Cannot return without format
        format!("{}", self.name)

        // self.name            // does not work as return statement
        // self.name.clone()    // works
    }

}


// Traits share similarities with interfaces in other languages
// Likewise, traits define method signatures
trait BasicInfo {
    fn summarize(&self) -> String;
}

// implementation for a trait
impl BasicInfo for Person {
    fn summarize(&self) -> String {
        format!("Summary:\n name: {}, age: {}", self.name, self.age)
    }
}

// implementation for a trait from another library/module
// made available
impl NameTrait for Person {
    fn my_name_is(&self) -> String {
        format!("My name is: {}", self.name)
    }
}

// Sub-Traits / Trait inheritance
// Requieres that implementing type needs to implement super-trait as well 
trait SubTrait: other_trait::SuperTrait {
    fn summarize_2(&self) -> String;
}

impl SubTrait for Person {
    fn summarize_2(&self) -> String {
        format!("Summary:\n name: {}, age: {}", self.name, self.age)
    }    
}

impl other_trait::SuperTrait for Person {
    fn am_i_right(&self) -> bool {
       true
    }
}


#[allow(unused_variables)] 
pub fn run() {
    println!("\nmodule structs: run() called \n---------");

    // Requires path separator (::) for access
    let other = other_struct::Accessible {
        a_bool: true,
        b_int: 1337,
    };
    
    let mut p = Person {
        age: 18,
        name: "Jane".to_string()
    };

    // composed Struct instantiation
    let p_composed = PersonComposed {
        age: 18,
        name: "Jane".to_string(),
        composed: Accessible {
            a_bool: true,
            b_int: 1,
        }
    };

    // cloning variables with the .. notation
    let clone = Person {
        name: "Jane (clone)".to_string(),
        ..p 
    };

    p.change_age(19);
    
    println!("Person {} {}", p.age, p.name);

    println!("Getting name: {}", p.get_name());
    println!("BasicInfo Trait {}", p.summarize());
    println!(" {}", p.my_name_is());
    println!("Getting age {}", p.get_age());

    println!("Super Trait {}", p.summarize_2());
    println!("Sub Trait {}", p.am_i_right());




    // Instantiate Numbers tuple struct
    let nums = Numbers(12, 100, 1000000);

    // Goes by index
    println!("Numb3rss {} {} {}", nums.0, nums.1, nums.2);

}