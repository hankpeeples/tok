use clap::Parser;
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dir: String,
}

fn main() {
    let args = Args::parse();

    let current_dir = env::current_dir().expect("pwd err");

    traverse_dir(current_dir, &args);
}

fn traverse_dir(path: PathBuf, args: &Args) {
    for entry in fs::read_dir(path).expect("read_dir err") {
        let entry = entry.expect("entry err");
        let meta = fs::metadata(entry.path()).expect("metadata err");

        if meta.is_file() {
            println!("{:?}", entry);
        } else {
            traverse_dir(entry.path(), &args);
        }
    }
}
