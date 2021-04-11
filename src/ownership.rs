// Ownsership 

pub fn run() {

    // Creating a vector with the macro "vec!"
    let v: Vec<u16> = vec![1,2,3,4,5];
    // v.push(1337);
    let vec2 = &v;

    // Q: Why does v require an ampersand but not in the next function call?

    // println!("{:?}", (v, vec2));

    let v: Vec<u16> = vec![1,2,3,4,5];
    let vec2: Vec<u16> = vec![0,0,0];
    let vec2 = &v;
    println!("{:?} {:?}", v, vec2);
    println!("{:?}", (&v, vec2));


    let v: Vec<u16> = vec![1,2,3,4,5];
    let vec2: Vec<u16> = vec![0,0,0];

    // The vectors are passed into a tuple, which consumes the data (move)
    println!("{:?}", (v, vec2));
    println!("{:?}", (v, vec2));

    let v: Vec<u16> = vec![1,2,3,4,5];
    let vec2: Vec<u16> = vec![0,0,0];
    // The macro seemingly does not consume the vector
    println!("{:?} {:?}", v, vec2);
    println!("{:?} {:?}", v, vec2);



}