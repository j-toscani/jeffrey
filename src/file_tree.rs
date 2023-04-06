use std::fs::{DirEntry, read_dir, ReadDir};

#[derive(Debug)]
pub enum FileTreeEntry {
    File(DirEntry), 
    Folder(Folder),
    Unreadable,
}

#[derive(Debug)]
pub struct Folder {
    entry: DirEntry,
    content: Vec<FileTreeEntry>
}

impl Folder {
    fn try_from_dir_entry(entry: DirEntry) -> Result<Folder, Box<dyn std::error::Error>> {
        let mut content: Vec<FileTreeEntry> = Vec::new();
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            todo!()
        }

        let directory = read_dir(entry.path())?;

        for entry in directory {
            if let Ok(ok_entry) = entry {
                if let Ok(inner_file_type) = ok_entry.file_type() {
                    if inner_file_type.is_dir() {
                        content.push(FileTreeEntry::Folder(Folder::try_from_dir_entry(ok_entry)?))
                    } else {
                        content.push(FileTreeEntry::File(ok_entry))
                    }
                }
            } else {
                content.push(FileTreeEntry::Unreadable);
            }
        }

        Ok(Folder { entry, content })
    }

    fn is_empty(&self) -> bool {
        self.content.len() > 0
    }
}

#[derive(Debug)]
pub struct FileTree {
    tree: Vec<FileTreeEntry>
}

impl FileTree {
    pub fn new(directory: ReadDir) -> FileTree {
        let mut tree: Vec<FileTreeEntry> = vec![];

        for entry in directory {
            if let Ok(ok_entry) = entry {
                if let Ok(file_type) = ok_entry.file_type() {
                    if file_type.is_dir() {
                        tree.push(FileTreeEntry::Folder(Folder::try_from_dir_entry(ok_entry).unwrap()));
                    } else {
                        tree.push(FileTreeEntry::File(ok_entry))
                    }
                }
            } else {
                tree.push(FileTreeEntry::Unreadable)
            }
        }

        FileTree { tree }
    }
}