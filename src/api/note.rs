use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::Path,
};

use sha_rs::{Sha, Sha1};

pub fn add(note: &String) {
    let contents = note.as_bytes();

    let hasher = Sha1::new();
    let hash = hasher.digest(contents);

    let path = Path::new(".notes/db").join(hash);

    let mut file = File::create(path).expect("File could not be created");
    file.write(contents).expect("Error writing file");
}

pub fn delete(hash: &String) {
    let path = Path::new(".notes/db").join(hash);

    match fs::remove_file(path) {
        Ok(_) => println!("Note deleted."),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("Note not found.");
        }
        Err(err) => {
            eprintln!("Unexpected error: {}", err);
        }
    }
}
  
pub fn list() {
    let path = Path::new(".notes/db");
    let directory = path.read_dir().expect("Could not read directory");

    for entry in directory {
        if let Ok(entry) = entry {
            println!("{}", entry.path().file_name().unwrap().display());
        }
    }
}
