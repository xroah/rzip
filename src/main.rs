use std::fs;

mod archive;
mod cmd;
mod error;

fn main() {
    let home = home::home_dir().unwrap();
    let dir = home.join(".rzip");

    if !dir.exists() {
        let _ = fs::create_dir(dir);
    }

    cmd::create_cmd();
}
