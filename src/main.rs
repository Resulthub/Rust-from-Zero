fn main() {
    // let name = String::from("Sadik kabir Ahmad");

    // println!("Character at index 6: {}", match name.chars().nth(6){
    //     Some(c) => c.to_string(),
    //     None => "Not found".to_string(),
    // })

    println!("Occupation is {}", match get_occupation("Ahmad"){
        Some(r) => r,
        None => "Not occupation found",
    })

}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Sadik" => Some("Programmer"),
        "Kabir" => Some("Programmer"),
        _ => None,
    }
}