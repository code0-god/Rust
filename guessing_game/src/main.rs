use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        // Strung: UTF-8 encoded bit of text.
        let mut guess = String::new(); // new instance of a String
        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // read_line returns Result - Ok or Err (enum)
    
        // parse() : convert type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}