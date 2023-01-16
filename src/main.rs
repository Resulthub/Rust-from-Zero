struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
fn main() {


}

fn give_two() -> i32 {
    2
}

#[cfg(test)]
mod demo_tests {

    #[test]
    #[should_panic]
    fn test_basic(){
        assert!(1 == 1); // OK
        panic!("Oh no!"); // OK

    }

    #[test]
    fn test_equal(){
        assert_eq!(super::give_two(), 1 + 1); // OK
        assert_ne!(super::give_two(), 1 + 2); // OK
    }

    #[test]
    fn test_structs(){
        let rect = super::Rectangle { 
            width: 10, 
            height: 10 
        };
        assert!(rect.is_square()); // OK
    }
} 