use std::env;

fn main() {
    // get the args
    let args: Vec<String> = env::args().collect();

    // reversing
    let data: Vec<String> = args.iter().rev().cloned().collect();
    for el in data {
        let reversed: String = el.chars().rev().collect();
        print!("{} ", reversed);
    }
    println!("");
}
