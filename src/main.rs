use std::env;

mod traversal;
use traversal::traverse_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().expect("pwd err");

    traverse_dir(current_dir);
}
