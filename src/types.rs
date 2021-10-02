/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)

U = Unsigned, it just means there is no negative values.
I = Integers (can be positive or negative)

Floats: f32, f64
Boolean (bool)

Characters (char)
char = single characters only

Tuples
tuples are basically lists

Arrays
arrays in rust are fixed length, vectors are "growable arrays"
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 9223372036854775807;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get Boolean from expression
  let is_greater: bool = 10 < 5;

  // Char unicode char
  let a1 = 'a';
  let eggplant = '🍆';
  let eggplant_unicode = '\u{1F346}';

  println!(
    "{:?}",
    (
      x,
      y,
      z,
      is_active,
      is_greater,
      a1,
      eggplant,
      eggplant_unicode
    )
  )
}
