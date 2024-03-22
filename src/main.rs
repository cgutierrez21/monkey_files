#![allow(warnings)]
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
- gtk-rs
- iced
- webview
- tempfile
- libp2p
*/

/*
Variables used throughout project
- home_directory: self explanatory
- current_location: which directory are we currently in
- content_list: what is in the current directory - one vector of directories one vector of files
- got_directories: the vector of directories from content_list
- got_files: the vector of files from content_list
*/
use home;
use std::io::Write;
use std::{env, path, io}; // , fs
mod file_explorer;
use file_explorer::fe_functions::*;
use file_explorer::fe_search::*;

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
    let (mut got_directories, mut got_files) = get_content(&current_location);

    // NOTE: Display those files/directories from vector
    println!(
        "Displaying the content of {}:",
        current_location.file_name().unwrap().to_str().unwrap()
    );
    println!(
        "This directory has {} directories and {} files.",
        got_directories.len(),
        got_files.len(),
    );

    println!("Directories:");
    for directory in got_directories {
        println!("{}", directory.file_name().unwrap().to_str().unwrap());
    }

    println!("Files: ");
    for files in got_files {
        println!("{}", files.file_name().unwrap().to_str().unwrap());
    }

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

    /* // NOTE: Create directory
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
    let _ = dir_content(&current_location); */

    println!("\n======================================");

    // NOTE: Rename directory/file

    /* println!("name changes");
    println!("Directory:");
    new_name(path::PathBuf::from(
        "/Users/cristiangutierrez/Pictures/Look At",
    ));
    println!("{}", current_location.to_str().unwrap());
    println!("File:");
    new_name(path::PathBuf::from(
        "/Users/cristiangutierrez/Pictures/LookAt/tomboy anime.jpg",
    ));
    println!("{}", current_location.to_str().unwrap()); */


    println!("\n======================================");
    let search_results = only_search_current(&current_location);
    if search_results.len() < 1 {
        println!("No results found.")
    } else {
        println!("{} results found.", search_results.len());
        for found in search_results {
            println!("{}", found.display());
        }
        println!("End of results.")
    }

    println!("\n======================================");

    print!("Enter filename to search: ");
    io::stdout().flush().unwrap();

    let mut search_term = String::new();

    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");

    search_term = search_term.trim().to_string().to_lowercase(); // This will remove the newline

    let new_search_results = start_search_current(&current_location, &search_term);

    if new_search_results.len() < 1 {
        println!("No results found.")
    } else {
        println!("{} resulsts found.", new_search_results.len());
        for found in new_search_results {
            println!("{}", found.display());
        }
        println!("End of results.")
    }

    println!("\n======================================");

    print!("Enter filename to search: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");

    search_term = search_term.trim().to_string().to_lowercase(); // This will remove the newline

    let new_search_results = system_search(&search_term);

    if new_search_results.len() < 1 {
        println!("No results found.")
    } else {
        println!("{} resulsts found.", new_search_results.len());
        for found in new_search_results {
            println!("{}", found.display());
        }
        println!("End of results.")
    }


    println!("Done");
}
