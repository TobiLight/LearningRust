use rand::Rng;
use std::{cmp::Ordering, io};

/**
 * Run guessing game
 * @params {u32} secret_number - random generated number
 * @params {i32} tries - number of tries
 */

fn start_game(secret_number: u32, mut tries: i32) {
    println!("Guess the number!");

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
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small!");
                tries = tries - 1;
                if tries == 0 {
                    println!("You are out of tries!\nPress\n N to continue playing\n Q to Quit game.");
                    break;
                }
            }
            Ordering::Equal => {
                println!("You Win! {} is the correct guess.", guess);
                println!("Press\n N to continue playing\n Q to Quit game.");
                break;
            }
            Ordering::Greater => {
                println!("Too Big!");
                tries = tries - 1;
                if tries == 0 {
                    println!("You are out of tries!\nPress\n N to continue playing\n Q to Quit game.");
                    break;
                }
            }
        }
    }
}

/**
 * This is the main program to run.
 */
fn main() {
    println!("Welcome to Guess...");
    println!("Press\n N to start New Game \n Q to Quit game.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let tries = 3;

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match &command.trim().to_lowercase() as &str {
            "n" => {
                // start new guessing game
                start_game(secret_number, tries)
            },
            "c" => {
                // continue playing guessing game
                start_game(secret_number, tries)
            },
            "q" => {
                // quit guessing game
                println!("Quitting game...");
                break;
            },
            _ => println!("Invalid command!\nPress N to start New Game or Q to Quit game.")
        }
    }    
}
