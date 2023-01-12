use std::collections::HashMap;
fn main() {
    let mut marks = HashMap::new();

    // Inserting values in HashMap
    marks.insert("Rust", 95);
    marks.insert("Python", 90);
    marks.insert("Java", 80);
    marks.insert("C", 85);

    // Find length of HashMap
    println!("How many Lanaguages have you studied? {}", marks.len()); 

    // Get a single value
    println!("What is your marks in Rust? {}", marks.get("Rust").unwrap());

    // Get match
    match marks.get("Python") {
        Some(mark) => println!("Your marks in Python is {}", mark),
        None => println!("You have not studied Python"),
    }

    // Check if a key exists
    if marks.contains_key("Java") {
        println!("You have studied Java");
    }

    // Check for value
    println!("Did you study C++ {}", marks.contains_key("C++"));

    // Loop through HashMap
    for (key, value) in &marks {
        println!("For {} you got {}%!", key, value);
    }

    // Remove a value
    marks.remove("C");
    println!("{:?}", marks);
    

}
