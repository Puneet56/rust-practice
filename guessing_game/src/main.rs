use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    let mut tries = 0;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        println!("Please input your guess.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low. Guess again"),
            Ordering::Greater => println!("Too high. Guess again"),
            Ordering::Equal => {
                println!(
                    "You guess correctly! Number is {}. You guessed in {} tries",
                    secret_number,
                    tries + 1
                );
                break;
            }
        }
        tries = tries + 1;
    }
}
