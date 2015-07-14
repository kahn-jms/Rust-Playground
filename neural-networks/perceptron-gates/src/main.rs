// Logical gates using perceptrons (neurons) to make weighted decisions

fn main() {

  let mut x1: i32;
  let mut x2: i32;

  //////////////////////////////////////
  // This is a horrible way to read int's
  // Need to split line and grab first two elements
  //////////////////////////////////////
  
  // Get first number
  println!("Input your first number:");

  // Start loop to get valid input
  loop {
    
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input)
      .ok()
      .expect("Failed to read line, try again.");

    // Reassign input as the first integer to process if valid int was input
    // Check input was an unsigned int
    x1 = match input.trim().parse::<i32>() {
      Ok(num) => num,
      Err(_) => {
        println!("Please type an integer.");
        continue;
      },
    };

    //println!("test: {}", x1);
    break;
    };

  // Get second number
  println!("Input your second number:");

  // Start loop to get valid input
  loop {

    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input)
      .ok()
      .expect("Failed to read line, try again.");

    // Reassign input as the first integer to process if valid int was input
    // Check input was an unsigned int
    x2 = match input.trim().parse::<i32>() {
      Ok(num) => num,
      Err(_) => {
        println!("Please type an integer.");
        continue;
      },
    };

    //println!("test: {}", x2);
    break;
    };

  //////////////////////////////////////
  // Binary Addition
  // Use NAND gates to add the two numbers input
  //////////////////////////////////////

  // Create NAND gate
  let nand_gate = |a,b| {
    if ( ((-2 * a) + (-2 * b) + 3) > 0 ) {
      return 1;
    } else {
      return 0;
    }
  };

  println!("NAND output:\t{}", nand_gate(x1,x2));

  let l1a = nand_gate(x1,x2);
  let l2a = nand_gate(x1,l1a);
  let l2b = nand_gate(x2,l1a);
  let l3a = nand_gate(l2a,l2b);
  
  let carry = |a| {
    if ((-4*a) + 3 > 0) {
      return 1;
    } else {
      return 0;
    }
  };

  println!("{} + {} = {}{}", x1, x2, carry(l1a), l3a);

  
  return;
}
