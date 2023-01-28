use std::{fs, path::PathBuf};

pub fn traverse_dir(path: PathBuf) {
    for entry in fs::read_dir(path).expect("read_dir err") {
        let entry = entry.expect("entry err");
        let meta = fs::metadata(entry.path()).expect("metadata err");

        if meta.is_file() {
            println!("{:?}", entry);
        } else {
            traverse_dir(entry.path());
        }
    }
}
