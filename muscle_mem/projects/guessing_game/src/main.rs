use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your guess:");

    //mut = mutable variable set to new empty instance of a string
    let mut guess = String::new();
    //pass &mut guess as the argument to read_line to tell it what string to store the user input in
    //references are immutable by default so we add mut
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
