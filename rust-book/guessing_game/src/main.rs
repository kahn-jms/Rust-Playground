// Guessing game


//use std::io;

// Main function
fn main() {

  let mut guess = String::new();

  println!("Make a guess:");

  std::io::stdin().read_line(&mut guess)
    .ok()
    .expect("Failed to read line");

  println!("You guessed: {}", guess);
}
