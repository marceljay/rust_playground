
pub struct Accessible {
    pub a_bool: bool,
    pub b_int: u16,
}

struct NotAccessible {
    pub a_bool: bool,
    pub b_int: u16,
}


// Traits share similarities with interfaces in other languages
// Likewise, traits define method signatures
pub trait NameTrait {
    fn my_name_is(&self) -> String;
}
