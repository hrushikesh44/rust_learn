use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Please type a number");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num ,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!🎉🍾");
                break;
            }
        };
    
    }
    
    
}