// Structs can be used to create custom data types

// Regular struct
struct Person {
    age: u8,
    name: String
    // name: &str, // needs lifetime?
}

// Tuple struct - unnamed fields
struct Numbers(u8, u16, usize);

// impl keyword is primarily used to define implementations on types
// e.g. member functions
impl Person {

    // Works with or without ampersand, so reference only needed for heap data?
    fn get_age(self) -> u8 {
        self.age
    }

    // Requires ampersand, because instance changes
    fn change_age(&mut self, new_age: u8) {
        self.age = new_age;
    }

    fn get_name(&self) -> String {
        // Cannot return without format
        format!("{}", self.name)

        // self.name            // does not work
        // self.name.clone()    // works
    }

}


pub fn run() {

    let mut p = Person {
        age: 18,
        name: "Jane".to_string()
    };

    p.change_age(19);
    
    println!("Person {} {}", p.age, p.name);

    println!("Getting name: {}", p.get_name());
    println!("Getting age {}", p.get_age());



    // Instantiate Numbers tuple struct
    let nums = Numbers(12, 100, 1000000);

    // Goes by index
    println!("Numb3rss {} {} {}", nums.0, nums.1, nums.2);

}