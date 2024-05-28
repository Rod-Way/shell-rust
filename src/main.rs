#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        let tokens = tokenize(input);

        match tokens[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
            _ => println!("{}: command not found", input),
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}
