use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("Welcome to the guesing game");

    loop {
        println!("Please input your number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
        
            let guess = guess.trim();
            if guess == "leave" {
                println!("Leaving...");
                break;
            }
            let guess: u32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input"); 
                    continue;
                }
            };
            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => { 
                    println!("You got it!");
                    break;
                }
            }
    }
}
