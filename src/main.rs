use core::num;
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    print!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input you guess.");
        
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {} ", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You wind!");
                break;
            }
            
        }
    }


}

