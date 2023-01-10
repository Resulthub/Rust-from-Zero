fn main() {
    print_numbers_to(20)


}

fn print_numbers_to(num: i32) {
    for i in 1..num {
        if is_even(i) {
            println!("{} is even", i)
        } else {
            println!("{} is odd", i)
        }
    }
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0
}