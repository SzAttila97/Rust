// Primitive string = Immutable fixed-length string somewhere in the moemory
// String = Groawable, heap-allocated data strucutre  that use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";                    // 1st type
    let mut hello2 = String::from("Hello World!"); // 2nd type

     println!("1st: {} , 2nd: {}", hello, hello2);

     // Get length
     println!("Length: {}", hello.len());

    //  hello2.push('W');                      // mut needed
    //  println!("Pushed: {}", hello2);

     hello2.push_str(" pushed more than one char.");
     println!("Pushed: {}", hello2);

     // Capacity in bytes  
     println!("Capicity: {}", hello2.capacity());

     println!("Is empty: {}", hello2.is_empty());

     // Contains
     println!("Conatains 'World' {}", hello2.contains("World"));

     // Replace
     println!("Replace: {}", hello.replace("World", "Thhere"));

     // Loop through string by whitespace
     for word in hello2.split_whitespace(){
         println!("{}", word);
     }

     // Create string with capacity
     let mut s = String::with_capacity(10);
     s.push('a');
     s.push('b');

     // Assertion testing
     assert_eq!(2, s.len());    // True
     // assert_eq!(2, s.len());    // Fail
     assert_eq!(10, s.capacity());

     println!("{}", s);

}