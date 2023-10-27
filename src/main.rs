use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("You guessed : {guess}"); 
    // can also be written as
    println!("You guessed : {}",guess);
}
