// TODO: look at each crate to see what to explore
/*
- hyper or reqwest
- tokio or async-std
- serde
- mime
- hyper-tls
- serde_json
- walkdir
- filetime
- gtk-rs or druid
- iced
- webview
- tempfile
- libp2p
*/
use home;
// use std::io::Write;
use std::{env, path}; // , fs, io
mod file_explorer;
use file_explorer::fe_functions::*;

fn main() {
    // setting up home directory
    let home_directory: path::PathBuf;
    if let Some(path) = home::home_dir() {
        home_directory = path;
    } else {
        eprintln!("Unable to determine home directory.");
        return;
    }
    println!("Your home directory: {}", home_directory.display());

    // setting up current directory
    let mut current_location = env::current_dir().unwrap();

    // NOTE: Display content in current directory
    println!(
        "Displaying the content of {}:",
        current_location.file_name().unwrap().to_str().unwrap()
    );
    let _ = dir_content(&current_location);

    println!("\n======================================");

    // NOTE: Vector with content of current directory
    let mut content_list = Vec::new();
    match get_content(&current_location) {
        Ok(content) => {
            content_list = content;
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }

    // NOTE: Display those files/directories from vector
    println!(
        "Displaying the content of {}:",
        current_location.file_name().unwrap().to_str().unwrap()
    );
    println!("\nDirectory contents:");
    for file in &content_list {
        println!("{}", file.file_name().unwrap().to_str().unwrap())
    }
    println!(
        "This directory has {} files/directories.",
        content_list.len()
    );

    println!("\n======================================");

    // NOTE: Go into one of the folders and display whats inside (controlled for now using Pictures directory)

    /* current_location = new_path(&content_list[2].to_str().unwrap().to_string());
    println!(
        "Files in {} directory:",
        current_location
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    );
    let _ = dir_content(&current_location); */

    println!("\n======================================");

    // NOTE: Go up a directory

    /* current_location.pop();
    println!(
        "Now in the {} directory:",
        current_location
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    );
    let _ = dir_content(&current_location); */

    // NOTE: Create directory
    create_directory(&current_location);
    // NOTE: Create directory and go into it after creation
    current_location = create_goto_directory(&current_location);
    println!(
        "Now in the {} directory:",
        current_location
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    );
    let _ = dir_content(&current_location);

    println!("\n======================================");
    println!("name changes");
    println!("Directory:");
    new_name(path::PathBuf::from("/Users/cristiangutierrez/Pictures/Look At"));
    println!("{}", current_location.to_str().unwrap());
    println!("File:");
    new_name(path::PathBuf::from("/Users/cristiangutierrez/Pictures/LookAt/tomboy anime.jpg"));
    println!("{}", current_location.to_str().unwrap());

    println!("Done");
}
