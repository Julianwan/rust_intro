// Variables hold primitive data or references to data
// Variables are immutable by default (you cannot re-assign them)
// Rust is a block-scoped language

pub fn run() {
  let name = "Julian";
  let mut age = 31;
  println!("My name is {} and I am {}", name, age);

  age = 32;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  // Usually all uppercase and add a type
  const ID: i32 = 001;

  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Julian", 31);

  println!("{} is {}", my_name, my_age);
}
