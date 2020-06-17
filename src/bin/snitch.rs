use vimsnitch::gitignore::Gitignore;

use std::path::Path;

fn main() {
    let mut current_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    current_dir.push_str("/.gitignore");
    let current_path = Path::new(&current_dir);
    let mut ignore = Gitignore::new(current_path);
    ignore.included_files();

    for file in ignore.get_files().iter() {
        println!("{}", file.to_str().unwrap());
    }
}
