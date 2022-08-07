use std::{io, process};
enum Actions {
    Flatten,
    Rename,
    None,
}
mod flatten;
mod rename;

pub fn run(config: Vec<String>) {
    let (action, args) = parse_config(config).expect("parse config failed");
    if let Actions::Flatten = action {
        match flatten::run(args.iter()) {
            Ok(()) => {
                println!("flatten success");
                process::exit(1)
            }
            Err(e) => {
                eprintln!("flatten failed: {}", e);
                process::exit(1)
            }
        }
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
        _ => Actions::None,
    };
    let args = config[2..].to_vec();
    Ok((action, args))
}
