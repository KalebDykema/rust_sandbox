// Constant variables can be defined globally (outside of main or another function), are not immutable, and must be annotated with a type.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {  
    println!("How many seconds are in 3 hours? {}", THREE_HOURS_IN_SECONDS);
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);


    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // In order to change the type of a variable and redefine it, best to not it mutable or you get an error
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Number of spaces: {}", spaces);
}
