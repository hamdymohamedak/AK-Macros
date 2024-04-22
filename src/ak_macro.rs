#[macro_export]
macro_rules! akp {
    // For a single string literal
    ($expr:expr) => {{
        let stdout = stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", $expr).expect("Failed to write to stdout");
    }};
    // For a format string with variable arguments
    ($fmt:expr, $($arg:tt)*) => {{
        let stdout = stdout();
        let mut handle = stdout.lock();
        writeln!(handle, $fmt, $($arg)*).expect("Failed to write to stdout");
    }};
}
#[macro_export]
macro_rules! p {
    ($expr:expr) => {
        write!(::std::io::stdout(), "{}", $expr).unwrap();
    };
}
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
#[macro_export]
macro_rules! remove_folder {
    ($path:expr) => {
        if let Err(e) = fs::remove_dir($path) {
            eprintln!("Error removing: {}", e);
        } else {
            println!("Done...!");
        }
    };
}
#[macro_export]
macro_rules! remove_file {
    ($path:expr) => {
        if let Err(e) = fs::remove_file($path) {
            eprintln!("Error removing: {}", e);
        } else {
            println!("Done...!");
        }
    };
}
#[macro_export]
macro_rules! remove_all_folders {
    ($path:expr) => {
        if let Err(e) = fs::remove_dir_all($path) {
            eprintln!("Error removing: {}", e);
        } else {
            println!("Done...!");
        }
    };
}

#[macro_export]
macro_rules! use_loop {
    ($should_execute:expr, $start_number:expr, $end_number:expr, $theVar:ident, $the_method:expr) => {
        for $theVar in $start_number..$end_number {
            if $should_execute {
                $the_method;
            }
        }
    };
}
