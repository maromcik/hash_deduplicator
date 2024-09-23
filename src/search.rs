use std::{fs, io};
use std::path::PathBuf;
use md5;


fn _list_files(vec: &mut Vec<PathBuf>, path: PathBuf) -> io::Result<()>  {
    if path.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result
        in paths {
            let full_path = path_result?.path();
            _list_files(vec, full_path)?;
        }
    } else {
        vec.push(path);
    }
    Ok(())
}

pub fn list_files<T: Into<PathBuf>>(path: T) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    let path = path.into();
    _list_files(&mut vec, path)?;
    Ok(vec)
}