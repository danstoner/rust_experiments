use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let min = 1;
    let max = 10;
    println!("Guess a number between {min} and {max}!");

    let secret_number: u32 = rand::thread_rng().gen_range(min..=max);

    loop {


        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println! ("Too big!"),
            Ordering::Equal => {
                println! ("You win! Secret was: {secret_number}");
                break;
            }
        }
    }
}