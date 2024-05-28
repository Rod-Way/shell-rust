use core::str;
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
        let tokens = input.split_whitespace().collect::<Vec<&str>>();

        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "exit" if tokens.len() == 2 => {
                let code = tokens[1].parse::<i32>().unwrap_or_else(|_| {
                    println!("exit: invalid exit code");
                    1
                });
                process::exit(code);
            }
            "echo" => {
                let args = &tokens[1..].join(" ");
                println!("{}", args)
            }
            _ => println!("{}: command not found", input),
        }
    }
}
