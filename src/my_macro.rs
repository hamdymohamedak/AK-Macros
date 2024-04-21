
//akp!("")
#[macro_export]
macro_rules! akp  {
    ($expr:expr) => {
        write!(::std::io::stdout(), "{}", $expr).unwrap();
    };
}
// askanderP!("")
#[macro_export]
macro_rules! askanderP  {
    ($expr:expr) => {
        write!(::std::io::stdout(), "{}", $expr).unwrap();
    };
}
// p!("")
#[macro_export]
macro_rules! p  {
    ($expr:expr) => {
        write!(::std::io::stdout(), "{}", $expr).unwrap();
    };
}
// input_prompt!("")
#[macro_export]
macro_rules! input_prompt {
    ($prompt:expr) => {{
        print!("{}", $prompt);
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }};
}
