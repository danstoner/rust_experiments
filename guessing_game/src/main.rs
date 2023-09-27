use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    const MIN: u32 = 1;
    const MAX: u32 = 10;
    let mut guesses: u32 = 0;
    println!("Guess a number between {MIN} and {MAX}!");

    let secret_number: u32 = rand::thread_rng().gen_range(MIN..=MAX);

    loop {


        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guesses += 1;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You guessed: {guess}");
                println!("Please type a number!");
                continue;
            }
        };
        if guess > MAX {
            println!("You guessed: {guess}");
            println!("Guess should be between {MIN} and {MAX}");
            continue;
        };



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println! ("Too big!"),
            Ordering::Equal => {
                println! ("You won! It took you {guesses} guesses!");
                break;
            }
        }
    }
}