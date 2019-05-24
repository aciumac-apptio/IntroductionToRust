pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    //Basic Formatting
    println!("Number: {} {}", 1, 3);

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Artem", "Moldova", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "baseball");

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}