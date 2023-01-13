mod code{
    fn chicken() {
        println!("Chicken !!");
    }

    pub fn print_message() {
        chicken();
        println!("How's it going today?");
    }

    pub mod water {
        pub fn print_message() {
            println!("Water is wet");
        }
    }

}
fn main(){
    code::print_message();
    
    code::water::print_message();
}