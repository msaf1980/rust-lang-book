extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        // Compare a number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! {} < {}", guess, secret_number),
            Ordering::Greater => println!("Too big! {} > {}", guess, secret_number),
            Ordering::Equal => {
                println!("You win! {} = {}", guess, secret_number);
                break;
            },
        }
    }
}
