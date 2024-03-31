#![allow(warnings)]
// TODO: symlink detection
// TODO: look at each crate to see what to explore
/*
- hyper or reqwest
- tokio or async-std
- serde
- mime
- hyper-tls
- serde_json
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
- got_directories: the vector of directories from content_list
- got_files: the vector of files from content_list
- search_term: self explanatory
- search_results: self explanatory
*/
use home;
use std::io::Write;
use std::{env, io, path}; // , fs
mod file_explorer;
use file_explorer::fe_functions::*;
use file_explorer::fe_input::*;
use file_explorer::fe_search::*;

fn main() {
    let home_directory: path::PathBuf;
    let mut current_location = env::current_dir().unwrap();
    let mut search_term = String::new();
    let mut got_directories = Vec::new();
    let mut got_files = Vec::new();
    // setting up home directory
    if let Some(path) = home::home_dir() {
        home_directory = path;
    } else {
        eprintln!("Unable to determine home directory.");
        return;
    }
    println!("Your home directory: {}", home_directory.display());

    // setting up current directory

    println!(
        "Displaying the content of {}:",
        current_location.file_name().unwrap().to_str().unwrap()
    );
    let _ = dir_content(&current_location);

    println!("\n======================================");

    // NOTE: Vector with content of current directory
    println!("get_content function");
    match get_content(&current_location) {
        Ok((directories, files)) => {
            got_directories = directories;
            got_files = files;
        }
        Err(err) => {
            println!("{}", err);
        }
    };

    println!(
        "Displaying the content of {}:",
        current_location.file_name().unwrap().to_str().unwrap()
    );

    if got_directories.is_empty() {
        println!(
            "Directory, {}, has no directories\n",
            current_location.file_name().unwrap().to_str().unwrap()
        );
    } else {
        println!("Directories:");
        for directory in got_directories {
            println!("{}", directory.file_name().unwrap().to_str().unwrap());
        }
    }

    if got_files.is_empty() {
        println!(
            "Directory, {}, has no files\n",
            current_location.file_name().unwrap().to_str().unwrap()
        );
    } else {
        println!("Files: ");
        for files in got_files {
            println!("{}", files.file_name().unwrap().to_str().unwrap());
        }
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

    /*
    create_directory(&current_location);
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

    println!("only_search_current function");
    search_term = user_input().to_lowercase(); // This will remove the newline

    let mut search_results = match only_search_current(&current_location, &search_term) {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

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

    println!("start_search_current function");
    search_term = user_input().to_lowercase(); // This will remove the newline

    search_results = match start_search_current(&current_location, &search_term) {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if search_results.is_empty() {
        println!("No results found.")
    } else {
        println!("{} results found.", search_results.len());
        for found in search_results {
            println!("{}", found.display());
        }
        println!("End of results.")
    }

    println!("\n======================================");

    // NOTE: system-wide search

    /* search_term = user_input().to_lowercase(); // This will remove the newline

    let new_search_results = match system_search(&search_term) {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if new_search_results.is_empty() {
        println!("No results found.")
    } else {
        println!("{} resulsts found.", new_search_results.len());
        for found in new_search_results {
            println!("{}", found.display());
        }
        println!("End of results.")
    } */

    println!("Done");
}
