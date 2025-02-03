use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let file_content = fs::read_to_string(file_path).expect("Error reading content of the file");

    println!("File Content:\n{file_content}");
}
