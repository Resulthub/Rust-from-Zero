struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
fn main() {
   let mut bg = Color { red: 255, green: 255, blue: 255 };

   bg.green = 45; // error: cannot assign to immutable field `bg.blue`

   println!("The background color is ({}, {}, {})", bg.red, bg.green, bg.blue);
}