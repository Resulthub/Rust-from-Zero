use std::io;
fn main() {
    let mut input = String::new();

    println!("Hey mate! Say something to me!");

    match io::stdin().read_line(&mut input){
        Ok(_)
        => {
            println!("You said: {}", input);
        },
        Err(e) => println!("Error: {}", e),
    }

}
