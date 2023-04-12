use std::{
    env::current_dir,
    fs::{read_dir, DirEntry, ReadDir},
    path::PathBuf,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum FileTree {
    File(DirEntry),
    Folder(Folder),
    Unreadable,
}

impl FileTree {
    pub fn copy_to(&self, to: &mut PathBuf) -> Result<()> {
        if to.is_absolute() {
            self.copy(to)?;
        } else {
            let mut current_dir = current_dir()?.canonicalize()?;
            current_dir.push(to);

            self.copy(&mut current_dir)?;
        }

        Ok(())
    }

    fn copy(&self, to: &mut PathBuf) -> Result<()> {
        match self {
            FileTree::File(entry) => {
                let mut file_path = to.clone();
                file_path.push(entry.file_name());
                std::fs::copy(entry.path(), file_path)?;
                Ok(())
            }
            FileTree::Folder(folder) => folder.copy(to),
            FileTree::Unreadable => Ok(()),
        }
    }

    pub fn new(directory: ReadDir) -> FileTree {
        let mut tree: Vec<FileTree> = vec![];

        for entry in directory {
            if let Ok(ok_entry) = entry {
                if let Ok(file_type) = ok_entry.file_type() {
                    if file_type.is_dir() {
                        tree.push(FileTree::Folder(
                            Folder::try_from_dir_entry(ok_entry).unwrap(),
                        ));
                    } else {
                        tree.push(FileTree::File(ok_entry))
                    }
                }
            } else {
                tree.push(FileTree::Unreadable)
            }
        }

        let folder = Folder {
            entry: None,
            content: tree,
        };
        FileTree::Folder(folder)
    }
}

#[derive(Debug)]
pub struct Folder {
    entry: Option<DirEntry>,
    content: Vec<FileTree>,
}

impl Folder {
    pub fn try_from_dir_entry(entry: DirEntry) -> Result<Folder> {
        let mut content: Vec<FileTree> = Vec::new();
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            todo!()
        }

        let directory = read_dir(entry.path())?;

        for entry in directory {
            if let Ok(ok_entry) = entry {
                if let Ok(inner_file_type) = ok_entry.file_type() {
                    if inner_file_type.is_dir() {
                        content.push(FileTree::Folder(Folder::try_from_dir_entry(ok_entry)?))
                    } else {
                        content.push(FileTree::File(ok_entry))
                    }
                }
            } else {
                content.push(FileTree::Unreadable);
            }
        }

        Ok(Folder {
            entry: Some(entry),
            content,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.content.len() > 0
    }

    fn copy(&self, to: &mut PathBuf) -> Result<()> {
        if let Some(entry) = &self.entry {
            to.push(entry.path());
        }

        std::fs::create_dir(to.clone())?;

        for entry in &self.content {
            entry.copy(to)?;
        }

        Ok(())
    }
}
