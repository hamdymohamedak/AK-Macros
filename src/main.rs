use std::fs;
use std::io::{stdin, stdout, Write};
mod ak_macro;
fn main() {
    // Rust with AK-macros is a easy to learn

    // to print use akp!("")
    akp!("hello world");

    // to get input from user use input_prompt!("")
    input_prompt!("Enter Your Age:");

    // to remove file use remove_folder!(add the path)
    remove_file!("/home/ak/Desktop/t");

    // to remove folder use remove_folder!("add the path")
    remove_folder!("/home/ak/Desktop/t");

    // (true,fromNumber,ToNumber,character,{method})
    //Example
    // use_loop!(true,0,100,i,{ println!("hello world") } )
    use_loop!(true, 0, 100, i, { println!("{} - Numbers", i) })
}
