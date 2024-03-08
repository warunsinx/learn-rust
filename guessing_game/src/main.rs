use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !");

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number !");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }

        println!("You guessed: {guess}");
    }
}
