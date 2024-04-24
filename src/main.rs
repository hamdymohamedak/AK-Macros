use std::fs;
mod lib;
fn main() {
    
    akp!("hello world");

    let my_name = input_prompt!("Enter the Name for Best crate. ?");

    if_cond!(
        my_name,
        my_name == "ak-macros",
        akp!("hello devs. ak-macros make rust be a simple"),
        akp!("Take your time I'm sure you will fall in love with ak-macors")
    );

    

}
