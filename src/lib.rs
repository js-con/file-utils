use std::{io, process};
mod flatten;
mod rename;
mod sort;
mod utils;

enum Actions {
    Flatten,
    Rename,
    Sort,
    None,
}

pub fn run(config: Vec<String>) {
    let (action, args) = parse_config(config).expect("parse config failed");
    match action {
        Actions::Flatten => flatten::run(args.iter()),
        Actions::Rename => rename::run(args.iter()),
        Actions::Sort => sort::run(args.iter()),
        Actions::None => println!("wrong action!"),
    }
}
fn parse_config(config: Vec<String>) -> Result<(Actions, Vec<String>), io::Error> {
    if config.len() <= 1 {
        eprintln!("Please input your action:\n 1.flat\n 2.rename ");
        process::exit(1)
    };
    let action = match config[1].as_str() {
        "flat" => Actions::Flatten,
        "rename" => Actions::Rename,
        "sort" => Actions::Sort,
        _ => Actions::None,
    };
    let args = config[2..].to_vec();
    Ok((action, args))
}
