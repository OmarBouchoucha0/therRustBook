use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    loop {
        println!("Guess The Number! Between 1 and 10");
        let mut guess = String::new();
        let target: u32 = rand::thread_rng().gen_range(1..=10);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) if (1..=10).contains(&num) => num,
            Ok(_) => continue,
            Err(_) => continue,
        };
        match guess.cmp(&target) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("YOU GUEESED CORRECT !!");
                break;
            }
        }
    }
}
