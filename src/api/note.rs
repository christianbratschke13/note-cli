use std::{fs::File, io::Write, path::Path};

use sha_rs::{Sha, Sha1};

pub fn add(note: &String) {
    let contents = note.as_bytes();

    let hasher = Sha1::new();
    let hash = hasher.digest(contents);

    let path = Path::new(".notes/db").join(hash);

    let mut file = File::create(path).expect("File could not be created");
    file.write(contents).expect("Error writing file");
}
