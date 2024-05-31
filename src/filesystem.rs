// use std::{
//     env, fs,
//     io::{self},
//     path::{self, Path},
// };

// pub fn cd(args: [&str]) -> io::Result<()> {
//     let the_path = args[0];
//     match the_path {
//         "~" | "$" | "$HOME" | "$USERPROFILE" => {
//             let home_dir = match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
//                 Ok(dir) => path::PathBuf::from(dir),
//                 Err(_) => {
//                     println!("error: home path not found");
//                     return Err(_);
//                 }
//             };
//             if let Err(_) = env::set_current_dir(&home_dir) {
//                 println!("error: failed to change directory to home");
//                 return Err(_);
//             }
//         }
//         _ => {
//             if let Ok(dir) = fs::canonicalize(Path::new(the_path)) {
//                 if dir.exists() && dir.is_dir() {
//                     if let Err(_) = env::set_current_dir(path::PathBuf::from(dir)) {
//                         println!("error: failed to change directory");
//                         return Err(_);
//                     }
//                 } else {
//                     println!("{}: No such file or directory", dir.display());
//                     return Err(_);
//                 }
//             } else {
//                 println!("{}: No such file or directory", the_path);
//                 return Err(_);
//             }
//         }
//     }
//     return Ok(());
// }

// pub fn ls() {}

// pub fn pwd() {}

// pub fn mkdir() {}

// pub fn rmdir() {}

// pub fn rm() {}

// pub fn touch() {}

// pub fn cp() {}

// pub fn mv() {}

// pub fn cat() {}

// pub fn head() {}

// pub fn tail() {}
