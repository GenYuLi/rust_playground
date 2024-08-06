use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

pub fn guess_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();
    loop {
        print !("Please input your guess: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let mut guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        
        match secret_number.cmp(&guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
        
