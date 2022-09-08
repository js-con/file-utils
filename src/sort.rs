use std::{fs, path::Path, process, slice::Iter};

use crate::{
    flatten,
    utils::{get_paths_in_dir, get_suffix},
};

fn parse_sort_args(mut args: Iter<String>) -> (&Path, bool) {
    if args.len() == 0 {
        eprintln!("please input target dir to sort");
        process::exit(1);
    }
    let target_dir = Path::new(args.next().expect("parse args failed"));
    let is_deep = args.any(|x| x == "--deep");

    (target_dir, is_deep)
}

pub fn run(args: Iter<String>) {
    let (target_dir, _) = parse_sort_args(args.clone());
    flatten::run(args);

    match get_paths_in_dir(target_dir) {
        Ok(files) => {
            for file in files {
                let suffix = match get_suffix(&file) {
                    Ok(s) => s.unwrap_or_else(|| "".to_string()),
                    Err(e) => {
                        eprintln!("{}", e);
                        "".to_string()
                    }
                };
                if suffix.is_empty() {
                    continue;
                }
                let category = target_dir.join(&suffix);
                if !category.is_dir() {
                    fs::create_dir(&category).expect("failed to create category");
                }
                fs::copy(&file, category.join(&file.file_name().unwrap()))
                    .expect("failed to copy file");
                fs::remove_file(&file).expect("failed to remove file after move");
            }
            println!("sort success!");
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    }
}
