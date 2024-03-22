use std::io::Write;
use std::{fs, io, path}; // env,

pub fn user_input() -> String {
    let mut search_term = String::new();
    print!("Enter filename to search: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");
    search_term = search_term.trim().to_string().to_lowercase(); // This will remove the newline

    search_term
}
