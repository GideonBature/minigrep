use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_query, file_path) = parse_config(&args);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With test:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    (query, file_path)
}
