// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let mut tx = String::from("Texan");

  // Get Length (will work for either type)
  println!("Length: {}", tx.len());

  // Push a single character
  tx.push('s');

  // Push a string
  tx.push_str(" are tough as nails!");

  // Get the capacity (number of bytes it can stores)
  println!("Capacity: {}", tx.capacity());

  // Check if it is empty
  println!("Is Empty: {}", tx.is_empty());

  // Check if it contains a substring
  println!("Contains 'nails': {}", tx.contains("nails"));

  // Replace a part of a string (does not mutate)
  println!(
    "Replace 'nails': {}",
    tx.replace("nails", "an iron curtain")
  );

  // Loop through string by whitespace
  for word in tx.split_whitespace() {
    println!("{}", word)
  }

  // Create string with capacity
  let mut j = String::with_capacity(10);

  // Push values onto the string with capacity
  j.push('j');
  j.push('g');
  j.push('w');

  // Assertion testing (will only alert if fails)
  assert_eq!(3, j.len());
  assert_eq!(10, j.capacity());

  println!("{}", j);

  println!("{}", tx)
}
