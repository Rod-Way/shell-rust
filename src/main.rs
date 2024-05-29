use core::str;
#[allow(unused_imports)]
use std::{
    env, fs,
    io::{self, Write},
    path::{self, Path},
    process,
};

fn main() {
    let builtins = vec!["exit", "echo", "type"];

    let path_env = env::var("PATH").unwrap_or_else(|_| "PATH not found".to_string());
    let paths: Vec<&str> = path_env.split(':').collect();

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
            "cd" if tokens.len() == 2 => {
                let the_path = tokens[1];
                match the_path {
                    "~" => {
                        let home_dir = match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
                            Ok(dir) => path::PathBuf::from(dir),
                            Err(_) => {
                                println!("error: home path not found");
                                continue;
                            }
                        };
                        env::set_current_dir(&home_dir).unwrap();
                    }
                    _ => {
                        let dir = fs::canonicalize(Path::new(the_path)).unwrap();
                        if !dir.exists() || !dir.is_dir() {
                            println!("{}: No such file or directory\\n", dir.display());
                            continue;
                        }
                        env::set_current_dir(dir).unwrap();
                    }
                }
            }
            "pwd" if tokens.len() == 1 => {
                println!("{}", env::current_dir().unwrap().display());
            }
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
            "type" if tokens.len() == 2 => {
                let command = tokens[1];
                if builtins.contains(&command) {
                    println!("{} is a shell builtin", command);
                } else {
                    let mut is_found = false;
                    for path in &paths {
                        let full_path = Path::new(path).join(command);
                        if full_path.exists() {
                            println!("{} is {}", command, full_path.display());
                            is_found = true;
                            break;
                        }
                    }
                    if !is_found {
                        println!("{} not found", command);
                    }
                }
            }
            _ => {
                let command = tokens[0];
                let mut is_found = false;

                for path in &paths {
                    let full_path = Path::new(path).join(command);
                    if full_path.exists() {
                        is_found = true;

                        let args = &tokens[1..];

                        let status = process::Command::new(full_path)
                            .args(args)
                            .status()
                            .expect("failed to execute process");

                        if !status.success() {
                            eprintln!("{}: command failed", command);
                        }
                        break;
                    }
                }

                if !is_found {
                    println!("{}: command not found", input)
                }
            }
        }
    }
}
