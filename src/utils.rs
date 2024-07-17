use std::io::{self, Write};

pub fn get_trick_input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase()
}
