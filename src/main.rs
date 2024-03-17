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
use std::{env, fs, io, path};

fn dir_content(path: &path::PathBuf) -> io::Result<()> {
    if !path.exists() {
        eprintln!("Directory doesn't exist!");
        return Ok(());
    }

    for element in fs::read_dir(path)? {
        let entry = element?;
        let path = entry.path();
        println!("{}", path.file_name().unwrap().to_str().unwrap());
    }
    Ok(())
}

fn get_content(path: &path::PathBuf) -> io::Result<Vec<path::PathBuf>> {
    let mut content = Vec::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        content.push(entry.path());
    }

    Ok(content)
}

fn new_path(given_path: &String) -> path::PathBuf {
    let updated_path: path::PathBuf = given_path.into();
    updated_path
}

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

    // display content in current directory
    println!("Displaying the content of {}:", current_location.file_name().unwrap().to_str().unwrap());
    let _ = dir_content(&current_location);

    println!("\n======================================");

    // vector with content of current directory
    let mut content_list = Vec::new();
    match get_content(&current_location) {
        Ok(content) => {
            /* for path in content {
                let current_file = path.file_name().unwrap().to_str().unwrap().to_string();
                content_list.push(current_file)
            } */
            content_list = content;
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }
    content_list.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });
    // content_list.sort();

    // display those files/directories from vector
    println!("Displaying the content of {}:", current_location.file_name().unwrap().to_str().unwrap());
    println!("\nDirectory contents:");
    for file in &content_list {
        println!("{}", file.to_str().unwrap())
    }
    println!(
        "This directory has {} files/directories.",
        content_list.len()
    );

    println!("\n======================================");

    // go into one of the folders and display whats inside (controlled for now using Pictures directory)
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

    // current_location.pop();
    current_location = "~/".into();
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

    println!("Done");
}
