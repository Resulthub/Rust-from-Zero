fn main() {
    let number = 1;

    match number {
        1 => println!("it is one!"),
        2 => println!("there is two of them!"),
        10 | 11 => println!("it is 10 or 11!"),
        _ => println!("it is something else!"),
    }
}
