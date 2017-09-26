extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Write guess number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Can not read string.");

        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(error) => {
                println!("{}", error.to_string());
                continue;
            }
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess number is less."),
            Ordering::Greater => println!("Your guess number is greater."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
