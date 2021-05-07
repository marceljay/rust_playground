// struct must be pub AND fields must be pub too
#[derive(Debug, Clone)] // proc - derive macro
pub struct Accessible {
    pub a_bool: bool,
    pub b_int: u16,
}


struct NotAccessible {
    pub a_bool: bool,
    pub b_int: u16,
}


// This trait will be "exported"
// possible because of 'pub' keyword
pub trait NameTrait {
    fn my_name_is(&self) -> String;
}

// Default implementation 'Default' is a keyword here
// Maybe defined trait in the prelude?
impl Default for Accessible {
    fn default() -> Self {
        Self {
            a_bool: true,
            b_int: 0,
        }
    }
}

 fn run() {
    // double colon because &self is omitted in function definition
    let def_struct = Accessible::default();

    // works only because we included the Debug macro for :?
    println!("{:?}", def_struct);
}
