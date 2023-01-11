struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My Name is {}, and I am {}", self.name, self.age);
    }
}

fn main() {
    let sadik = Person {
        name: String::from("Sadik Kabir Ahmad"),
        age: 25,
    };

    println!("{}", sadik.to_string() );

}
