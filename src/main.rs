use std::io;

fn main() {
    print!("Guess the number!");
    println!("Please input you guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    print!("You guessed: {}", guess);


}
