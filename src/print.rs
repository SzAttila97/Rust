pub fn run() {
    // Print to console 
    println!("Hello from the print.rs file");

    //println!(1);  => Error 
    println!("Place holder needed! {}", 1); 
    println!("Same thing with {}", "String");

    // Positional Arguments
    println!("I can say {0} like {0} so I can save some {1} with it.", "this", "time");

    // Named Argument  
    println!("{name} likes to play {activity}.", name = "Andrew", activity = "soccer");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10 ,10);

    //Placeholder for debug trait
    println!("{:?}", (10, true, "String"));

    //Basic math
    println!("10 + 10 = {}", 10+10);
    println!("10 - 10 = {}", 10-10);
    println!("10 * 10 = {}", 10*10);
    println!("10 / 10 = {}", 10/10);
    
}