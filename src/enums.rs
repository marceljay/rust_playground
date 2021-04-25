// Enums or enumerations are another custom data type in rust
// Example here is based on the rust book/docs
// In rust each enum variant can have data to go along with it.


// An empty enum
enum EmptyEnum { }

// An enum containing different variants
enum WebEvent {
    // Basic variants (are like exist vs. non-exist, on/off)
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

// function can be called with any WebEvent enum
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
    println!("module enums: run() called \n---------");

    // variants of the enums are namespaced via ::
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    // includes an anonymous struct
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

// An example from the Rust book. 
// IpAddrKind tells us the kind, we can combine it with a struct to have data too
// I assume this must be more memory efficient that using a string
// and more readable than e.g. a boolean array or bitset
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}s9