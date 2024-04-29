mod lib;
fn main() {
    akp!("Hello World");

    let os = this_OS!();
    let year = this_year!();
    if os == "linux" && year == 2024  {
        let my_name = input_prompt!("Enter Your Name: ");
        if_cond!(
            my_name,
            my_name == "hamdy",
            open_Web!("https://askander.vercel.app"), // if
            println!("else cond working") // else
        )
    } else {
        akp!("The Name not equal Hamdy Try Again...");
        akp!("This year is not the current year, the Current Year is: {}",year);
    }
}
