use clap::{Args as ClapArgs, Parser, Subcommand};
use std::{env, path::PathBuf};

mod add;
mod del;
mod ls;
mod open;
mod unzip;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(version)]
struct Args {
    files: Vec<PathBuf>,
    #[arg(short)]
    password: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(ClapArgs, Debug)]
struct DelOrAddArgs {
    files: Vec<PathBuf>,
    #[arg(short)]
    target: PathBuf,
    #[arg(short)]
    password: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Unzip {
        files: Vec<PathBuf>,
        #[arg(short)]
        password: Option<String>,
    },
    Open {
        file: PathBuf,
        #[arg(short)]
        password: Option<String>,
    },
    Add(DelOrAddArgs),
    Del(DelOrAddArgs),
    Ls {
        files: Vec<PathBuf>,
        #[arg(short, default_value_t = false)]
        recursive: bool,
    },
}

pub fn create_cmd() {
    let args = Args::parse();

    for f in &args.files {
        println!("File: {}", f.to_str().unwrap_or("None"))
    }

    match args.command {
        Some(cmd) => {
            println!("Command: {:?}", cmd);

            match cmd {
                Commands::Add(DelOrAddArgs {
                    files,
                    target,
                    password,
                }) => {}
                Commands::Del(DelOrAddArgs {
                    files,
                    target,
                    password,
                }) => {}
                Commands::Ls { files, recursive } => {
                    let _ = ls::list(files, recursive);
                }
                Commands::Open { file, password } => {}
                Commands::Unzip { files, password } => {}
            }
        }
        None => {
            println!("No command provided.");
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        println!("Current directory: {:?}", current_dir)
    }
}
