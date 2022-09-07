use crate::utils::{extract_dir, get_files_in_dir};
use std::{fs, path::Path, process, slice::Iter};

pub fn run(args: Iter<String>) {
    match parse_flatten_args(args) {
        Some((target_dir, new_dir, is_deep)) => {
            match get_files_in_dir(target_dir) {
                Ok(files) => {
                    for dir in files {
                        if dir.is_dir() {
                            extract_dir(&dir, target_dir, is_deep)
                                .map_err(|e| eprintln!("error when extract dir:{}", e));
                            fs::remove_dir_all(&dir)
                                .map_err(|e| eprintln!("error when remove dir: {}", e));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
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
