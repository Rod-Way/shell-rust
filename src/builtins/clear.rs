use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    write!(stdout, "\x1B[2J\x1B[H").unwrap();
    stdout.flush().unwrap();
}
