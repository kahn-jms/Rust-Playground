// Guessing game

extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

// Main function
fn main() {

  
  // Generate a random number (u32?)
  let secret_num = rand::thread_rng().gen_range(1, 101);
  // Let's find out
  // Can't get this to work yet
  //format!("{:?}", secret_num);


  // Start a loop while user guesses number
  loop {
    
    println!("Make a guess:");

    // The guess will be read as a string
    let mut guess = String::new();

    // Read user's guess
    std::io::stdin().read_line(&mut guess)
      .ok()
      .expect("Failed to read line");

    // Check input was an unsigned int
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please type an integer.");
        continue;
      },
    };
    
    println!("You guessed: {}", guess);

    // Compare guess with generated number
    match guess.cmp(&secret_num) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("Correct!");
        break;
        },
    }
  }

  
  println!("The secret number was:\t{}", secret_num);

  return;
}
