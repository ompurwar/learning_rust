use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::{self, Colorize};

fn main() {
    println!("Guess the NUmber!");
    let secret_number = rand::thread_rng().gen_range(1, 1000);
    
    println!("The secret NUmber is: {}", secret_number);
    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guesses: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;
            }
        }
    }
}
