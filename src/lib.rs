use std::{
    env, fs, io,
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
    process,
    slice::Iter,
};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Please input your action:\n 1.flat\n 2.rename ");
        process::exit(1);
    };
    let action = &args[1];
    if *action == "flat".to_string() {
        match flatten(args[2..].into_iter()) {
            Ok(_) => println!("flatten success"),
            Err(e) => {
                eprintln!("flatten failed: {}", e);
                process::exit(1)
            }
        }
    }
}
pub fn flatten(mut args: Iter<String>) -> Result<(), io::Error> {
    match args.next() {
        Some(target_path) => {
            let target_dir = Path::new(target_path);
            let inner_dirs: Vec<PathBuf> = fs::read_dir(target_path)?
                .map(|f| f.unwrap().path())
                .filter(|p| p.is_dir())
                .collect();
            for dir in inner_dirs {
                extract_dir(dir.as_path(), target_dir)?;
            }
            if let Some(new_name) = args.next() {
                fs::rename(target_dir, new_name)?;
            };
        }
        None => {
            eprintln!("Please input which folder you want to flatten");
        }
    }
    Ok(())
    // if let Some(arg_layer) = args.next() {}
    // !unimplemented!()
}
pub fn extract_dir(from: &Path, to: &Path) -> Result<(), io::Error> {
    let files: Vec<PathBuf> = fs::read_dir(from)
        .expect("error when read_dir")
        .map(|f| f.unwrap().path())
        .collect();
    for file in files {
        fs::copy(&file, Path::join(&to, &file.file_name().unwrap())).expect("failed copy file");
    }
    fs::remove_dir_all(from)?;
    Ok(())
}
