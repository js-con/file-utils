use std::env;
use file_utils::run;

fn main() {
    let config: Vec<String> = env::args().collect();
    run(config);
}
