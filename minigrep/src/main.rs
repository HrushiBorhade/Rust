use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let file_content =
        fs::read_to_string(config.file_path).expect("Error reading content of the file");

    println!("File Content:\n{file_content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
