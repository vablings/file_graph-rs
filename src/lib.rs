use std::fmt;
use std::{fs, path::Path};

type Result<T> = std::result::Result<T, FileTreeError>;

#[derive(Debug, Clone)]
enum FileTreeError {
    SymlinkError(String),
    UnknownEntryType,
    FilePathError,
}

impl fmt::Display for FileTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileTreeError::SymlinkError(T) => write!(f, "Symlink found {T}"),
            FileTreeError::UnknownEntryType => write!(f, "Entry is not a File, Folder or Symlink"),
            FileTreeError::FilePathError => write!(f, "Failed to coerce &Path into String"),
        }
    }
}

#[derive(Debug)]
struct Entry {
    name: String,
    kind: EntryKind,
}

#[derive(Debug)]
enum EntryKind {
    File,
    Folder(Vec<Entry>),
}

fn read_entry(path: &Path) -> Result<Entry> {
    let name = path
        .file_name()
        .ok_or(FileTreeError::FilePathError)?
        .to_str()
        .ok_or(FileTreeError::FilePathError)?
        .to_string();

    let ty = fs::metadata(path).unwrap().file_type();
    if ty.is_file() {
        Ok(Entry {
            name,
            kind: EntryKind::File,
        })
    } else if ty.is_dir() {
        Ok(Entry {
            name,
            kind: EntryKind::Folder(read_folder(path)?),
        })
    } else if ty.is_symlink() {
        Err(FileTreeError::SymlinkError(name))
    } else {
        Err(FileTreeError::UnknownEntryType)
    }
}

fn read_folder<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        out.push(read_entry(&entry.path())?);
    }
    Ok(out)
}



