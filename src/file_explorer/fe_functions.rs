// use home;
use std::io::Write;
use std::{fs, io, path}; // env,

pub fn dir_content(path: &path::PathBuf) {
    if !path.exists() {
        eprintln!("Directory doesn't exist!");
        return;
    }

    for element in fs::read_dir(path).unwrap() {
        let entry = element.unwrap();
        let path = entry.path();
        println!("{}", path.file_name().unwrap().to_str().unwrap());
    }
}

pub fn get_content(path: &path::PathBuf) -> (Vec<path::PathBuf>, Vec<path::PathBuf>) {
    let mut directories = Vec::new();
    let mut files = Vec::new();
    for entry in fs::read_dir(&path).unwrap() {
        let current_entry = entry.unwrap().path();

        if current_entry.is_dir() {
            directories.push(current_entry);
        } else if current_entry.is_file() {
            files.push(current_entry);
        }
    }

    directories.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });

    files.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });
    (directories, files)
}

/* pub fn new_path(given_path: String) -> path::PathBuf {
    let updated_path: path::PathBuf = given_path.into();
    updated_path
} */

pub fn create_directory(path: &path::PathBuf) {
    print!("Enter name of new directory: ");
    io::stdout().flush().unwrap();

    let mut directory_name = String::new();

    io::stdin()
        .read_line(&mut directory_name)
        .expect("Failed to read line");
    directory_name = directory_name.trim().to_string(); // This will remove the newline

    let _ = fs::create_dir(path.join(path::PathBuf::from(&directory_name)));
}

pub fn create_goto_directory(path: &path::PathBuf) -> path::PathBuf {
    print!("Enter name of new directory: ");
    io::stdout().flush().unwrap();

    let mut directory_name = String::new();

    io::stdin()
        .read_line(&mut directory_name)
        .expect("Failed to read line");
    directory_name = directory_name.trim().to_string(); // This will remove the newline

    let _ = fs::create_dir(path.join(path::PathBuf::from(&directory_name)));

    path.join(path::PathBuf::from(&directory_name))
}

pub fn new_name(path: path::PathBuf) {
    let old_filename = path.file_name().unwrap().to_str().unwrap();
    let mut parent_path = path.clone();
    parent_path.pop();

    println!("Do not write file extension");
    print!("What do you want to call {}: ", old_filename);
    io::stdout().flush().unwrap();

    let mut new_filename = String::new();

    io::stdin()
        .read_line(&mut new_filename)
        .expect("Failed to read line");
    new_filename = new_filename.trim().to_string(); // This will remove the newline

    if !path.is_dir() {
        println!("Getting extension.");
        new_filename.push('.');
        new_filename.push_str(path.extension().unwrap().to_str().unwrap());
    }

    // let result = fs::rename(parent_path.join(old_filename), parent_path.join(new_filename));
    let _ = fs::rename(
        parent_path.join(old_filename),
        parent_path.join(&new_filename),
    );
}
