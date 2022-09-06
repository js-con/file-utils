use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
    slice::Iter,
};

pub fn run(args: Iter<String>) {
    match parse_flatten_args(args) {
        Some((target_dir, new_dir, is_deep)) => {
            for dir in get_files_in_dir(target_dir) {
                if dir.is_dir() {
                    if let Err(e) = extract_dir(&dir, target_dir, is_deep) {
                        eprintln!("error when extract dir: {}", e);
                    }
                    if let Err(e) = fs::remove_dir_all(&dir) {
                        eprintln!("error when remove dir: {}", e);
                    }
                }
            }
            if !new_dir.is_empty() {
                if let Err(e) = fs::rename(target_dir, new_dir) {
                    eprintln!("error when rename dir: {}", e)
                }
            }
            println!("flatten success");
        }
        None => {
            eprintln!("please input the target dir");
            process::exit(1);
        }
    }
}
fn parse_flatten_args(mut args: Iter<String>) -> Option<(&Path, &str, bool)> {
    let target_dir = Path::new(args.next()?);
    let other_args = &args.map(|s| s.as_ref()).collect::<Vec<&str>>();

    let new_dir = if !other_args.is_empty() && !other_args[0].contains("--") {
        other_args[0]
    } else {
        ""
    };
    let is_deep = other_args.contains(&"--deep");
    Some((target_dir, new_dir, is_deep))
}
fn extract_dir(from: &Path, to: &Path, deep: bool) -> Result<(), io::Error> {
    for file in get_files_in_dir(from) {
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
fn copy_dir(target: &Path, to: &Path) -> Result<(), io::Error> {
    if !target.is_dir() {
        Err(io::Error::new(io::ErrorKind::Other, "not a directory"))
    } else {
        let new_dir = Path::join(to, target.file_name().unwrap());
        fs::create_dir(&new_dir)?;

        for file in get_files_in_dir(target) {
            if file.is_dir() {
                copy_dir(&file, &Path::join(&new_dir, file.file_name().unwrap()))?;
            } else {
                fs::copy(&file, Path::join(&new_dir, file.file_name().unwrap()))?;
            }
        }
        Ok(())
    }
}
fn get_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("failed to read dir")
        .map(|f| f.unwrap().path())
        .collect()
}
