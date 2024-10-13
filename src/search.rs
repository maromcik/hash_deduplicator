use std::path::PathBuf;
use std::{fs, io};

fn _list_files(vec: &mut Vec<PathBuf>, path: PathBuf) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }
    if path.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            _list_files(vec, full_path)?;
        }
    } else {
        vec.push(path);
    }
    Ok(())
}

pub fn list_files<T: Into<PathBuf>>(paths: Vec<T>) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    for path in paths {
        let path = path.into();
        _list_files(&mut vec, path)?;
    }
    Ok(vec)
}
