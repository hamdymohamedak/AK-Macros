use std::io::{stdin, stdout, Write};
mod my_macro;
fn main() {
    akp!("hello AK_Macros\n");
    let user_input = input_prompt!("Please enter some text:");
    println!(": {}", user_input);
    if user_input == "hamdy" {
        println!("hello {}", user_input)
    } else {
        println!("user not avilable")
    }
}
