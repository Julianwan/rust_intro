// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let mut gay = String::from("Homosexual");

  // Get Length (will work for either type)
  println!("Length: {}", gay.len());

  // Push a single character
  gay.push('s');

  // Push a string
  gay.push_str(" rule the world!");

  // Get the capacity (number of bytes it can stores)
  println!("Capacity: {}", gay.capacity());

  // Check if it is empty
  println!("Is Empty: {}", gay.is_empty());

  // Check if it contains a substring
  println!("Contains 'world': {}", gay.contains("world"));

  // Replace a part of a string (does not mutate)
  println!("Replace 'world': {}", gay.replace("world", "gates of hell"));

  // Loop through string by whitespace
  for word in gay.split_whitespace() {
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

  println!("{}", gay)
}
