use std::path::PathBuf;

use file_tree::FileTree;
mod file_tree; 

fn main() {
    let dir = std::fs::read_dir("./src").unwrap();
    let tree = FileTree::new(dir);
    let mut to = PathBuf::from("src/hello");
    tree.copy_to(&mut to).unwrap();
}
