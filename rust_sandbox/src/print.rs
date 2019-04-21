pub fn run() {
    //Printing to Main
    println!("Hi let me give you a brief introduction of myself.");

    //Basic Formatting
    println!("{} have {} friend.", "I", 1);

    //Positional Arguments
    println!("Number of {1} {0} give: {2}", "I", "shits", 0);

    //Named Arguments
    println!("{name} likes to play {activity}", name = "James", activity = "chess");

    //Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Debug Trait Placeholder
    println!("{:?}", (12, true, "hello"));
    
    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}