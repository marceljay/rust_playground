// Enums or enumerations are another custom data type in rust
// Example here is based on the rust book/docs
// In rust each enum variant can have data to go along with it.


enum WebEvent {
    // Basic variants 
    PageLoad,
    PageUnload,
    // Variants that hold data
    KeyPress(char),
    SmallNumba(u8),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
    // LotsOfThings {
    //     usual_struct_stuff: bool,
    //     blabla: String,
    // }
}

// An empty enum
enum EmptyEnum { }


fn inspect(event: WebEvent) {
    // Match is similar to a switch/case
    // NOTE: ALL possible cases must be handled
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),

        // Destructure `var` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::SmallNumba(n) => println!("passed number '{}'.", n),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

pub fn run() {
    println!("module enums: run() called");

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;
    let numba   = WebEvent::SmallNumba(77);

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    inspect(numba);

}