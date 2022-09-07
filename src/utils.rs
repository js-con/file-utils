use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn extract_dir(from: &Path, to: &Path, deep: bool) -> Result<(), io::Error> {
    for file in get_files_in_dir(from)? {
        if file.is_dir() {
            if deep {
                extract_dir(&file, to, deep)?;
            } else {
                copy_dir(&file, to)?;
            }
        } else {
            fs::copy(&file, Path::join(to, &file.file_name().unwrap()))?;
        }
    }
    Ok(())
}
pub fn copy_dir(target: &Path, to: &Path) -> Result<(), io::Error> {
    if !target.is_dir() {
        Err(io::Error::new(io::ErrorKind::Other, "not a directory"))
    } else {
        let new_dir = Path::join(to, target.file_name().unwrap());
        fs::create_dir(&new_dir)?;

        for file in get_files_in_dir(target)? {
            if file.is_dir() {
                copy_dir(&file, &Path::join(&new_dir, file.file_name().unwrap()))?;
            } else {
                fs::copy(&file, Path::join(&new_dir, file.file_name().unwrap()))?;
            }
        }
        Ok(())
    }
}
pub fn get_files_in_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(dir).and_then(|files| files.map(|f| Ok(f?.path())).collect())
}
