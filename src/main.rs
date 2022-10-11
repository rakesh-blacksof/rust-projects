use colored::Colorize;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Please Enter Your Guess.");
    let secret_ans = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error Reading the line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please Enter A Valid Number".red());
                continue;
            }
        };
        match guess.cmp(&secret_ans) {
            Ordering::Less => println!("{}", "Enter A Higher Number".cyan()),
            Ordering::Equal => {
                println!("{}", "You Have Won".green());
                break;
            }
            Ordering::Greater => println!("{}", "Enter a Smaller Number".blue()),
        }
    }
}
