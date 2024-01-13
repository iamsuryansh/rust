use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guessing game");
    println!("Please input your guess. ");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number.");
            continue
        },
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too Small".red()),
        Ordering::Equal => {
            println!("{}", "Congratulations! You got it right!".green());
            break;
        },
        Ordering::Greater => println!("{}", "Too Large".red()),
    }
}

}
