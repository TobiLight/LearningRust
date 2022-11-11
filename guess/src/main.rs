use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 3;

    loop {
        println!("Tries Left: {}\nPlease input your guess.", tries);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number!!!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small!");
                tries = tries - 1;
                if tries == 0 {
                    println!("You are out of tries!");
                    break;
                }
            },
            Ordering::Equal => {
                println!("You Win! {} is the correct guess.", guess);
                break;
            }
            Ordering::Greater => {
                println!("Too Big!");
                tries = tries - 1;
                if tries == 0 {
                    println!("You are out of tries!");
                    break;
                }
            },
        }
    }
}