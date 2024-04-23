use std::fs;
use std::io::{stdin, stdout, Write};
mod ak_macro;
fn main() {
    // Rust with AK-macros is a easy to learn

    // // to print use akp!("")
    akp!("hello world");

    // // to get input from user use input_prompt!("")
    let x: String = input_prompt!("Enter Your Age:");

    // // to remove file use remove_folder!(add the path)
    remove_file!("/home/ak/Desktop/t");

    // // to remove folder use remove_folder!("add the path")
    remove_folder!("/home/ak/Desktop/t");

    // // (true,fromNumber,ToNumber,character,{method})
    // //Example
    use_loop!(true, 0, 100, i, { println!("{} - Numbers", i) });

    // use if_cond to type a shortcut for if condation
    let x = 5;
    if_cond!(
        x,
        x == 5,
        println!("x == Five"),  // if
        println!("x not Five")  // else
    );

    // simple app
    let my_name: String = input_prompt!("Enter your name: ");
    if_cond!(
        &my_name,
        &my_name == "hamdy",
        println!("You are hamdy"),                   //if
        remove_folder!("/home/ak/Desktop/Projects")  // else
    );
}
