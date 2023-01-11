fn main() {
    let mut my_string = String::from("how's it going? my name is Sadik");

    //Length of the string
    println!("Length: {}", my_string.len());

    //Is the string empty?
    println!("String is empty: {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    //Contains
    println!("Does the string Contains 'Sadik' {}", my_string.contains("Sadik"));

    my_string.push_str(" and I am learning Rust");

    println!("{}", my_string);
}
