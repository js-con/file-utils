#[cfg(test)]
mod test {
    use std::{env, fs, path::Path};

    #[test]
    fn test() {
        let current_dir = env::current_dir().expect("");
        println!("current_dir is {:?}", &current_dir);
        let target_dir = fs::create_dir(Path::join(&current_dir, "shit")).expect("");
    }
}
