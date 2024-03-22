use home;
use super::fe_functions::get_content;
use std::io::Write;
use std::{io, path};

// TODO: search in current directory
pub fn only_search_current(path: &path::PathBuf) -> Vec<path::PathBuf> {
    let (got_directories, got_files) = get_content(path);

    print!("Enter filename to search: ");
    io::stdout().flush().unwrap();

    let mut search_term = String::new();

    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");
    search_term = search_term.trim().to_string().to_lowercase(); // This will remove the newline

    let mut results: Vec<path::PathBuf> = Vec::new();

    if got_directories.len() > 0 {
        for directory in got_directories {
            let name = directory.to_str().unwrap().to_lowercase();
            if name.contains(&search_term) {
                results.push(directory.to_owned());
            }
        }
    }

    if got_files.len() > 0 {
        for files in got_files {
            let name = files.to_str().unwrap();
            if name.contains(&search_term) {
                results.push(files.to_owned());
            }
        }
    }

    results
}

pub fn start_search_current(path: &path::PathBuf, search_term: &String) -> Vec<path::PathBuf> {
    let (got_directories, got_files) = get_content(path);

    let mut results: Vec<path::PathBuf> = Vec::new();

    if got_directories.len() > 0 {
        for directory in got_directories {
            let name = directory
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase();
            if name.contains(search_term) {
                results.push(directory.to_owned());
            }
            let mut temp = start_search_current(&directory, &search_term);
            results.append(&mut temp);
        }
    }

    if got_files.len() > 0 {
        for files in got_files {
            let name = files.file_name().unwrap().to_str().unwrap().to_lowercase();
            if name.contains(search_term) {
                results.push(files.to_owned());
            }
        }
    }

    let mut directory_results: Vec<path::PathBuf> = Vec::new();
    let mut file_results: Vec<path::PathBuf> = Vec::new();

    for paths in results {
        if paths.is_dir() {
            directory_results.push(paths);
        } else {
            file_results.push(paths);
        }
    }

    directory_results.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });
    file_results.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });

    let mut final_results: Vec<path::PathBuf> = Vec::new();
    final_results.append(&mut directory_results);
    final_results.append(&mut file_results);

    final_results
}

pub fn system_search(search_term: &String) -> Vec<path::PathBuf> {
    let path: path::PathBuf = home::home_dir().unwrap();

    let (got_directories, got_files) = get_content(&path);

    let mut results: Vec<path::PathBuf> = Vec::new();

    if got_directories.len() > 0 {
        for directory in got_directories {
            let name = directory
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase();
            if name.contains(search_term) {
                results.push(directory.to_owned());
            }
            let mut temp = start_search_current(&directory, &search_term);
            results.append(&mut temp);
        }
    }

    if got_files.len() > 0 {
        for files in got_files {
            let name = files.file_name().unwrap().to_str().unwrap().to_lowercase();
            if name.contains(search_term) {
                results.push(files.to_owned());
            }
        }
    }

    let mut directory_results: Vec<path::PathBuf> = Vec::new();
    let mut file_results: Vec<path::PathBuf> = Vec::new();

    for paths in results {
        if paths.is_dir() {
            directory_results.push(paths);
        } else {
            file_results.push(paths);
        }
    }

    directory_results.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });
    file_results.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });

    let mut final_results: Vec<path::PathBuf> = Vec::new();
    final_results.append(&mut directory_results);
    final_results.append(&mut file_results);

    final_results
}
