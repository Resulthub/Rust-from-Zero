use std::process::Command;

fn main() {

    //python file.py

    let mut cmd = Command::new("python");
    cmd.arg("./file.py");

    // Execute the command

    match cmd.output() {
        Ok(o) => {
            // Print the output
            unsafe{

                println!("The output was: {}", String::from_utf8_unchecked(o.stdout));
            }
        }

        Err(e) => {
            // Print the error
            println!("There was an error! {}", e)
        }
    }


}