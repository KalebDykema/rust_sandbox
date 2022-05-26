fn main() {
    // // This creates a mutable string. Because this is mutable, we don't know the size! This is actually stored on the heap instead of the stack.
    // let mut s = String::from("I'm a mutable string!");
    // println!("{}", s);
    // s.push_str(" See?");
    // println!("{}", s);

    // {
    //     let s = String::from("I'm limited to this scope and will disappear");

    //     // The above variable gets "droped" from memory at the end of this block. Imagine the follow line is being run
    //     // drop s;
    // }

    // let x = String::from("Hello World");
    // let y = x;

    // // This won't work, as this value is now out of scope and was dropped from memory after re-assigning it to y.
    // // println!("{}", x);
    // println!("{}", y);

    // let x = String::from("Hello World");
    // let y = x.clone();

    // // This will work, only because we cloned it up above
    // println!("{}", x);
    // println!("{}", y);

    // let x = 3;
    // let y = x;

    // // This will work without copying, because the size is known at compilation time
    // println!("{}", x);
    // println!("{}", y);

    let x = "This works";
    let y = x;

    // This will also work without copying, because the size is known at compilation time
    println!("{}", x);
    println!("{}", y);
}
