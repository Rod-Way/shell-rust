use std::env;

fn main() {
    // get the args
    let args: Vec<String> = env::args().collect();

    // let args = &args.join(" ");
    println!("{}", args[1..].join(" "));
}
