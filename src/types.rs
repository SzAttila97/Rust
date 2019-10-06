/*
 Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 
 u => unsign => can't be negative
number of bits they take in memory
Floats: f32, f64
Boolean (bool)
Characters (char) => one character not string!


Rust is a statically typed language, which means that it must know the types of all
 variables at compile time, however, the compiler can usually infer what type we 
 want to use based on the value and how we use it.
 */

pub fn run(){
  // Default is "i32"
  let x = 1;

  // Defoult is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 56123123123;

  //Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i32: {}", std::i64::MAX);

  // Boolean
  let is_true: bool = true;  // we can leave bool here as well
  println!("{:?}", (x, y, z, is_true));

  // Get boolean from expression
  let is_greater: bool = 10 > 5;
  println!("{:?}", is_greater);

  // Character
  let a1 = 'a'; // Single quate in case of char
  //let a1 = 'ab; => Error couse there are 2 chars
  let face = '\u{1F602}';	

  println!("{:?}", (a1,face));
}