use glob::glob;
use std::{fs, path::PathBuf, process, slice::Iter};

fn parse_rename_args(args: Iter<String>) -> (Vec<PathBuf>, String) {
    let args = &args.map(|s| s.as_ref()).collect::<Vec<&str>>();
    if args.is_empty() {
        eprintln!("please input path");
        panic!()
    }
    let mut path_arr = vec![];
    for entry in glob(args[0]).expect("path should be glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    path_arr.push(path)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    let new_suffix;
    if args.len() == 2 {
        if args[1].starts_with('.') {
            new_suffix = args[1]
                .split("")
                .filter(|&s| s != ".")
                .collect::<Vec<&str>>()
                .join("");
        } else {
            new_suffix = args[1].to_string();
        }
    } else {
        new_suffix = "".to_string();
    };
    (path_arr, new_suffix)
}

pub fn run(args: Iter<String>) {
    let (path_arr, new_suffix) = parse_rename_args(args);
    for path in path_arr {
        let mut new_path = path.to_str().unwrap().split('.').collect::<Vec<&str>>();
        if new_path.len() < 2 {
            //no suffix
            if new_suffix.is_empty() {
                process::exit(1)
            }
            let new_path = new_path[0].to_string() + "." + &new_suffix;
            match fs::rename(&path, &new_path) {
                Ok(()) => println!("rename {:?} to {}", &path, &new_path),
                Err(e) => eprintln!("failed to rename file {:?}, {}", &path, e),
            }
        } else {
            let len = new_path.len();
            if new_suffix.is_empty() {
                new_path.pop().unwrap();
            } else {
                new_path[len - 1] = &new_suffix;
            }
            let new_path = new_path.join(".");
            match fs::rename(&path, &new_path) {
                Ok(()) => println!("rename {:?} to {}", &path, &new_path),
                Err(e) => eprintln!("failed to rename file {:?}, {}", &path, e),
            }
        }
    }
}
