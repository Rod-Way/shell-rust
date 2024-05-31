use std::{
    env, fs,
    io::{self, Write},
    path::{self, Path},
    process,
};

// mod filesystem;

fn main() {
    let builtins = vec!["exit", "ls", "type", "pwd", "cd", "clear"];

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src\\builtins");
    let builtins_path = fs::canonicalize(current_dir).unwrap();
    let path_env = env::var("PATH").unwrap_or_else(|_| "".to_string());
    let mut paths: Vec<_> = env::split_paths(&path_env).collect();
    paths.push(builtins_path);

    // setting uppdated PATH
    env::set_var("PATH", env::join_paths(paths.clone()).unwrap());

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
                let the_path = &tokens[1];

                let mut parts: Vec<&str> = the_path.split('/').collect();

                let home_dir = match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
                    Ok(dir) => path::PathBuf::from(dir),
                    Err(_) => {
                        println!("error: home path not found");
                        return;
                    }
                };

                match parts.get(0) {
                    Some(&"~") | Some(&"$") | Some(&"$HOME") => {
                        // let home_dir = env::var("HOME")
                        //     .or_else(|_| env::var("USERPROFILE"))
                        //     .unwrap();
                        parts[0] = &home_dir.to_str().unwrap();
                    }
                    _ => {}
                }
                let new_path = parts.join("/");

                if let Ok(dir) = fs::canonicalize(Path::new(&new_path)) {
                    if dir.exists() && dir.is_dir() {
                        if let Err(_) = env::set_current_dir(&dir) {
                            println!("error: failed to change directory");
                        }
                    } else {
                        println!("{}: No such file or directory", dir.display());
                    }
                } else {
                    println!("{}: No such file or directory", new_path);
                }
            }
            "ls" => {
                if tokens.len() == 1 {
                    for el in fs::read_dir(env::current_dir().unwrap()).unwrap() {
                        let entry_path = el.unwrap().path();
                        if let Some(file_name) = entry_path.file_name() {
                            if let Some(name_str) = file_name.to_str() {
                                println!("{}", name_str);
                            }
                        }
                    }
                } else if tokens.len() == 2 {
                    continue;
                }
            }

            "pwd" => {
                println!(
                    "{}",
                    env::current_dir()
                        .unwrap()
                        .display()
                        .to_string()
                        .trim_start_matches(r"\\?\")
                )
            }
            "exit" if tokens.len() == 2 => {
                let code = tokens[1].parse::<i32>().unwrap_or_else(|_| {
                    println!("exit: invalid exit code");
                    1
                });
                process::exit(code);
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
            "01110010" => {
                println!("(■_■¬) made by RodWay")
            }
            _ => {
                let command = tokens[0];
                let mut is_found = false;

                for path in &paths {
                    let full_path = Path::new(path).join(command.to_string() + ".exe");
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
