// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Andrew";
    let mut age = 22;
    println!("My name is {} and I am {}.", name, age);
    age = 23;   // without mut => cannot assign twice to immutable variable
    println!("My name is {} and I am {}.", name, age);

    // Constants
    const ID : i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Andrew", 22);
    println!("{} is {}.my_name", my_name, my_age);

}