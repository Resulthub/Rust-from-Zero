fn main(){
    // Replace
    {
        let my_string = String::from("Rust is awesome!");
        println!("After replace: {}", my_string.replace("awesome", "great"));
    }

    // Lines

    {
        let my_string = String::from("The quick brown fox \njumps over \nthe lazy dog.");
        for line in my_string.lines() {
            println!("[ {} ]", line);
        } 
    }

    // Split

    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed+the+video!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[0]);    
    }

    // Trim

    {
        let my_string = String::from("  My name is Sadik kabir Ahmad  \n\r");
        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    // Trim matches

    {
        let my_string = String::from("Rust is awesome!");
        println!("Before trim matches: {}", my_string);
        println!("After trim matches: {}", my_string.trim_matches('!'));
    }


    // Chars
    {
        let my_string = String::from("Rust is awesome!");

        println!("{}", my_string);

        // Get characters at index

        match my_string.chars().nth(3) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4")
        }
    }

}