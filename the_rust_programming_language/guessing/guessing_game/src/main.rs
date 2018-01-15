extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number (1 to 100)");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => print!("Too small\n"),
            Ordering::Greater   => print!("Too large\n"),
            Ordering::Equal     => {
                print!("You win\n");
                break;
            }
        }
    }
}
