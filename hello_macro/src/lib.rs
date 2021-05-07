// For the purpose of writing a custom derive (proc) macro

// The derive macro requires a crate "crate_name_derive"
// Both crates will need to be published separately, 
// and programmers using these crates will need to
// add both as dependencies and bring them both into scope

// The procedural macro here shall generate an implementation of 
// the HelloMacroTrait _trait_ for the _type_ the user annotated

pub trait HelloMacroTrait {
    fn hello_macro();
}


mod my_module {
    mod sub_module {}
}