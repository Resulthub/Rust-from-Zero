#![allow(dead_code)]

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match *self {
            Day::Saturday | Day::Sunday => false,
            _ => true
        }
    }
}

fn main() {

    let d = Day::Sunday;

    println!("Is Sunday a weekday? {}", d.is_weekday());
}