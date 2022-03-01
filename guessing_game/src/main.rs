// Import packages
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Create a random number between 1 and 100. 101 could also be =101.
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
    
        // Create a mutable variable
        let mut guess = String::new();
    
        // Take input from the terminal and set it to guess, or fail on an error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Turn the input into a mutable number. If it's not a valid number, start the loop over.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {}", guess);
    
        // Compare the guess with the secret number. If correct, end the loop. Otherwise, inform the user how close they were then start the loop over again.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // End the loop, thus ending the program.
                break;
            } 
        }
    }
}
