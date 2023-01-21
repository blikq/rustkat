use std::{io,str,fs, env};
use std::process::Command;

const _HELP: &str = "
Rkat, Cat written in Rust

Usage:
    rkat [OPTOINS]

OPTIONS:
    -- -V, -- --version       Print version info
    <file directory>    Print content of file

more options coming soon...
";

const _VERSION: &str = "rkat this version `0.1.0`";


fn main() {

    let args: Vec<String> = env::args().collect();
    let mut ppath = String::new();
    match args.len(){
        1 => {
            println!("Enter the path to the file you want to read:");
            io::stdin().read_line(&mut ppath)
                .expect("did not receive input");
            read(&ppath);
        },
        2 => match args[1].as_str() {
            _ => read(&args[1])
        },
        _ => println!("Features not yet implemented")
    }
}

fn read(path: &String) {

    // Open the file at the specified path
    let mut file = fs::File::open(path.as_str().trim()).unwrap();

    // Declare a variable to store the file contents
    let mut contents = Vec::new();

    // Read the contents of the file into the 'contents' variable
    io::Read::read_to_end(&mut file, &mut contents).unwrap();

    //Convert the bytes to a string
    let s = match str::from_utf8(&contents) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    // Convert the &str to a String
    let my_string = s.to_string();

    // Print the contents of the file
    Command::new("tput").arg("clear").output().expect("Failed to clear terminal");
    // print!("\x1B[2J");
    println!("\n");
    println!("{}", my_string);
}
