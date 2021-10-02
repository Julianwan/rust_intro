pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Single Value Output
  println!("Number: {}", 69);

  // Basic Formatting
  println!("{} is from {}", "Julian", "Austin");

  // Positional Arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Julian", "Austin", "code"
  );

  // Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "Julian",
    activity = "volleyball"
  );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "male"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10)
}
