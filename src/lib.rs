/// ```
/// use akp_macros::akp;
/// /// or
/// use ak_macros::*
/// akp!("Hello World");
/// ```
#[macro_export]
macro_rules! akp {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}
///```
/// ///input_prompt can get input from user
/// ///example
///  use apk_macros::input_prompt
/// /// or
/// use ak_macros::*
///
///let user_name = input_prompt!("Enter Your Name:");
/// akp!("User Name is:{}",user_name);
/// ```
#[macro_export]
macro_rules! input_prompt {
    ($prompt:expr) => {{
        use std::io::{stdin, stdout, Write};
        print!("{}", $prompt);
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string() // Changed parse() to to_string() to handle strings
    }};
}

///```
/// /// this macro can remove any folder at your PC
/// /// this macro take one value this value is folder Path
/// /// example
/// remove_folder!("/home/ak/Desktop/new_Folder/");
/// /// when macro remove folder the macro will print Done...! at terminal
/// ```
#[macro_export]
macro_rules! remove_folder {
    ($path:expr) => {
        use std::fs;
        if let Err(e) = fs::remove_dir($path) {
            eprintln!("Error removing: {}", e);
        } else {
            println!("Done...!");
        }
    };
}

///```
/// /// this macro can remove any file at your PC
/// /// this macro take one value this value is folder Path
/// /// example
/// remove_file!("/home/ak/Desktop/new_file/");
/// /// when macro remove file the macro will print Done...! at terminal
/// ```
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

///```
/// /// remove_all_folders macro can remove all folders and file at the path you will set it
/// remove_all_folders!("/home/ak/Desktop/new_Folder/");
/// ```
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

///```
/// ///use_loop macro can create a loop
/// ///this macro take Five values
/// /// 1 - if you want to do event at loop set True else set False
/// /// 2 - start-Number
/// /// 3 - End-Number
/// /// 4 - variable for loop
/// /// 5 - the method
/// /// you should type true at first value to method working
/// /// example
/// use_loop!(true, 0, 100, i, akp!("{}", i));
/// /// dont do this
///  use_loop!(false, 0, 100, i, akp!("{}", i)); /// Syntax Error
/// ```
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

///```
/// /// if_cond macro do if condition method
/// /// the macro take 4 values
/// /// 1 - import the value you want to do event about him
/// /// 2 - do condition
/// /// 3 - if event
/// /// 4 - else event
/// /// for example
/// let my_name = "ak-macros";
/// if_cond!(my_name,
/// my_name == "ak-macros",
/// akp!("Hello package"),
/// akp!("Oh No, this is not best crate...!")
/// )
/// /// let's do simple app with AK-Macros
///
/// let my_name = input_prompt!("Enter this Crate Name: ");
/// if_cond!(my_name,
/// my_name == "ak-macros",
/// akp!("Hello devs...!"),
/// akp!("Try Again...")
///);
///
/// ///let's use the use_loop macro at this app
///let my_name = input_prompt!("Enter this Crate Name: ");
///if_cond!(
///my_name,
///my_name == "ak-macros",
///use_loop!(true, 0, 100, i, akp!("{} - hello guys", i)),
/// akp!("Try Again...")
///);
/// // if you Type at input => ak-macros the use loop will start working
/// ```
#[macro_export]
macro_rules! if_cond {
    ($var:expr, $condition:expr, $first_method:expr, $last_method:expr) => {
        if $condition {
            $first_method
        } else {
            $last_method
        }
    };
    ($name:expr, $cond:expr, $action:expr) => {{
        if $cond {
            $action;
        }
    }};
    ($name:expr, $($cond:expr => $action:expr),*) => {{
        $(if $cond { $action; })*
    }};
    ($name:expr, $($cond:expr => $action:expr),* $(,)?) => {{
        $(if $cond { $action; })*
    }};
}

///```
/// /// terminal macro its like a terminal at your OS
/// /// the macro can do any command at you OS
/// /// to use the macro you should type two Values
/// /// 1 - your OS command
/// /// if you use windows the OS command is CMD
/// /// if you use Linux the OS command is sh
/// /// for example
/// terminal!("sh", "mkdir ak-macros_App");
/// /// if you want type more command you must end the command with ;
/// ///
///terminal!(
///"sh",
///"
///cd '/home/ak/Desktop/' ;
///mkdir ak-macros_App ;
///cd ak-macros_App ;
///touch new_file
///"
///);
/// ```
#[macro_export]
macro_rules! terminal {
    ($lang:expr, $($command:tt)*) => {
        let _ = std::process::Command::new($lang)
            .arg("-c")
            .arg(concat!($($command)*))
            .output()
            .expect("failed to execute process");
    };
}
///```
/// this_OS!(); ///macro can know your operating system Name
/// /// you can use it at any conditions
/// /// example
/// akp!("My operating system is :{}",this_OS!());
/// /// this command will print Your OS
///
/// ```
#[macro_export]
macro_rules! this_OS {
    () => {
        std::env::consts::OS
    };
}
///```
/// /// use_rand macro, this macro can generate a Random Number
/// /// to use the macro you should add rand at cargo.toml
/// /// the macro take 3 values
/// /// 1 - variable
/// /// 2 - point number
/// /// 3 - the Method
/// /// you can type to values only [1,3]
/// /// and the default value for point number is // i32
/// /// example
///use_rand!(my_float, {
///println!("Random float: {}", my_float);
///});
/// /// or
///use_rand!(my_float,f64,{
///println!("Random float: {}", my_float);
///});
/// ```
#[macro_export]
macro_rules! use_rand {
    ($var:ident, {$($method:tt)*}) => {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let $var: i32 = rng.gen();
        $($method)*
    };

    ($var:ident, $kind:ty, {$($method:tt)*}) => {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let $var: $kind = rng.gen();
        $($method)*
    };
}

///```
/// /// use_fetch it's a fetch api macro
/// /// to use it add
/// /// 1 - reqwest
/// /// 2 - tokio
/// /// 3 - serde
/// /// 4 - serde_json
/// /// at Cargo.toml
/// /// usage
/// /// the macro take two values
/// use_fetch!("api",HTTP_Request);
/// /// example
/// #[tokio::main]
///async fn main() -> Result<(), Box<dyn std::error::Error>> {
// Call the macro with API endpoint and HTTP method
///use_fetch!("https://fakestoreapi.com/products/", GET);

// Return Ok(()) as the result
/// Ok(())
///}
///
///
/// ```
#[macro_export]
macro_rules! use_fetch {
    ($api:expr, $method:ident) => {{
        extern crate reqwest;
        use std::collections::HashMap;
        let api = $api.to_string();
        let method = stringify!($method).to_string();

        let response = reqwest::Client::new()
            .request(reqwest::Method::from_bytes(method.as_bytes())?, &api)
            .send()
            .await?;

        let resp_json = response
            .json::<Vec<HashMap<String, serde_json::Value>>>()
            .await?;

        for item in resp_json {
            println!("{:#?}", item);
        }

        Ok(()) as Result<(), Box<dyn std::error::Error>>
    }};
}

///```
/// use_zip!(); /// macro can extract the ZIP files
/// /// to use this macro you should add zip-extract at Cargo.toml
/// /// the macro take two values
/// use_zip!("ZIP_PAth","Path_for_EXtract_Files");
///
/// /// example
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///use_zip!(
///"Desktop/folder.zip",
/// "Desktop/Extract_Folder"
///);

/// /// Return Ok(()) as the result
///Ok(())
///}

/// ```
///

#[macro_export]
macro_rules! use_zip {
    ($zip_file_location:expr, $extract_file_location:expr) => {{
        use ak_macros::use_zip;
        use std::fs::File;
        use std::io::Cursor;
        use std::io::{self, Read};
        use std::path::PathBuf;
        use zip_extract::ZipExtractError;
        let input_value = $zip_file_location;
        let target_path = $extract_file_location;

        let file = std::fs::File::open(input_value.trim())?;
        let mut bytes = Vec::new();
        file.take(1024 * 1024 * 100) // Limit the file size to 100 MB for safety
            .read_to_end(&mut bytes)?;

        let target = std::path::PathBuf::from(target_path.trim());

        let cursor = std::io::Cursor::new(bytes);

        match zip_extract::extract(cursor, &target, false) {
            Ok(_) => {
                println!("Extraction successful!");
                Ok(())
            }
            Err(err) => match err {
                zip_extract::ZipExtractError::Io(io_err) => Err(io_err),
                other_err => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Zip extraction error: {:?}", other_err),
                )),
            },
        }
    }};
}

///```
///  this_month!(); /// Macro to show month
/// /// example
/// let month = this_month!();
///  akp!("month is: {}",month);
/// ```   
#[macro_export]
macro_rules! this_month {
    () => {{
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seconds_since_epoch = since_epoch.as_secs();
        let remaining_seconds = seconds_since_epoch % (365 * 24 * 60 * 60);
        let month = remaining_seconds / (30 * 24 * 60 * 60) + 1;
        month
    }};
}

///```
///  this_year!(); /// Macro to show Year
/// /// example
/// let year = this_year!();
///  akp!("Year is: {}",year);
/// ```  
#[macro_export]
macro_rules! this_year {
    () => {{
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seconds_since_epoch = since_epoch.as_secs();
        let _ = seconds_since_epoch % (365 * 24 * 60 * 60);
        let year = 1970 + seconds_since_epoch / (365 * 24 * 60 * 60);
        year
    }};
}
///```
/// /// open_web macro can open a url at browser
/// ///the macro take one value this value is the URL Link
/// ///example
/// open_Web!("https://askander.vercel.app");
/// ```
#[macro_export]
macro_rules! open_Web {
    ($website_url:expr) => {{
        let url = $website_url;

        let result = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd").arg("/C").arg("start").arg(url).spawn()
        } else if cfg!(target_os = "macos") {
            std::process::Command::new("open").arg(url).spawn()
        } else {
            std::process::Command::new("xdg-open").arg(url).spawn()
        };

        // Check if the command executed successfully
        match result {
            Ok(_) => println!("Successfully opened website."),
            Err(e) => eprintln!("Failed to open website: {}", e),
        }
    }};
}


///```
/// /// macro can convert the string from lowerCase to UpperCase
///   use_upper_case!(name);
///   println!("{}",name);
/// ```
#[macro_export]
macro_rules! use_upper_case {
    ($input:ident) => {
        $input = $input.to_uppercase();
    };
}

///```
/// /// macro can convert the string from upperCase to lowerCase
///    use_lower_case!(name);
///   println!("{}",name);
/// ```
#[macro_export]
macro_rules! use_lower_case {
    ($input:ident) => {
        $input = $input.to_lowercase();
    };
}

///```
/// /// macro can create a file and add text at him
/// /// example
///fn main() -> std::io::Result<()> {
///use_createFile!("ak2","/home/ak/Desktop","Hello from ak-macros");
///Ok(())
/// }
/// /// the macro take three value
/// /// 1 - File name
/// /// 2 - File Path
/// /// 3 - Text
/// ```
#[macro_export]
macro_rules! use_createFile {
    ($file_name:expr, $file_path:expr,$input_text:expr) => {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::{Path, PathBuf}; // Import PathBuf for constructing paths

        let file_path = PathBuf::from($file_path).join($file_name); // Construct the file path
        let mut file = File::create(&file_path)?; // Use ? to propagate errors
        file.write_all($input_text.as_bytes())?; // Use ? to propagate errors
    };
}
