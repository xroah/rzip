
mod cmd;
mod error;
mod archive;

fn main() {
    let Some(home_dir) = home::home_dir() else {
        return;
    };
    let Some(home_str) = home_dir.to_str() else {
        return;
    };
    println!("Home: {}", home_str);

    cmd::create_cmd();
}