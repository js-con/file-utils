use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
    slice::Iter,
};

enum Actions {
    Flatten,
    Rename,
    None,
}

pub fn run(config: Vec<String>) {
    let (action, args) = parse_config(&config).expect("parse config failed");
    match action {
        Actions::Flatten => match flatten(args.iter()) {
            Ok(()) => {
                println!("flatten success");
                process::exit(1)
            }
            Err(e) => {
                eprintln!("flatten failed: {}", e);
                process::exit(1)
            }
        },
        _ => (),
    }
}
fn parse_config(config: &Vec<String>) -> Result<(Actions, Vec<String>), io::Error> {
    if config.len() <= 1 {
        eprintln!("Please input your action:\n 1.flat\n 2.rename ");
        process::exit(1)
    };
    let action: Actions;
    let args: Vec<String>;

    match config[1].as_str() {
        "flat" => action = Actions::Flatten,
        "rename" => action = Actions::Rename,
        _ => action = Actions::None,
    }
    args = config[2..].to_vec();
    Ok((action, args))
}
fn flatten(mut args: Iter<String>) -> Result<(), io::Error> {
    if let Some(target_path) = args.next() {
        let target_dir = Path::new(target_path);
        let inner_dirs: Vec<PathBuf> = fs::read_dir(target_path)?
            .map(|f| f.unwrap().path())
            .filter(|p| p.is_dir())
            .collect();
        for dir in inner_dirs {
            extract_dir(dir.as_path(), target_dir, true)?;
            fs::remove_dir_all(dir)?;
        }
        if let Some(new_name) = args.next() {
            fs::rename(target_dir, new_name)?;
        };
    }
    Ok(())
}
fn extract_dir(from: &Path, to: &Path, deep: bool) -> Result<(), io::Error> {
    let files: Vec<PathBuf> = fs::read_dir(from)
        .expect("error when read_dir")
        .map(|f| f.unwrap().path())
        .collect();

    for file in files {
        if file.is_dir() && deep {
            extract_dir(&file, &to, deep)?;
        } else {
            fs::copy(&file, Path::join(&to, &file.file_name().unwrap())).expect("failed copy file");
        }
    }
    Ok(())
}
