use std::fs::{self, DirEntry, ReadDir};

use file_tree::FileTree;
mod file_tree; 

fn main() {
    let dir = fs::read_dir(".").unwrap();
    let tree = FileTree::new(dir);

    println!("{:?}", tree);
}

fn print_dir_entry(entry: DirEntry, level: usize) {
    let indents = vec!["--"; level];
    println!("{}{}", indents.join(""), entry.file_name().to_str().unwrap());
}

fn read_dir_entries(dir: ReadDir, level: usize) {
    for el in dir {
        let entry = el.expect("Error while looping over dir entries");
        let file_type = entry.file_type().expect("Could not get file_type");
        
        if file_type.is_dir() {
            if let Ok(new_dir) = fs::read_dir(entry.path()) {
                read_dir_entries(new_dir, level + 1);
            }
        }
        print_dir_entry(entry, level);
    }
}
