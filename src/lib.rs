use std::{env, fs, io, process, slice::Iter};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Please input your action:\n 1.flat\n 2.rename ");
        process::exit(1);
    };
    let action = &args[1];
    if *action == "flat".to_string() {
        flatten(args[2..].into_iter())
    }
}
pub fn flatten(mut args: Iter<String>) {
    match args.next() {
        Some(folder) => {
            let mut result = fs::read_dir(".")
                .expect("error when read_dir")
                .map(|f| f.expect("error when read file").path());
            for file in result {
               println!("get file {:?}",file.file_name()); 
            }
        }
        None => {
            eprintln!("Please input which folder you want to flatten");
            process::exit(1)
        }
    }
    if let Some(arg_name) = args.next() {};
    if let Some(arg_layer) = args.next() {}
}
