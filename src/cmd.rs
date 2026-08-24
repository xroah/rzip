use clap::{Args as ClapArgs, Parser, Subcommand};
use std::path::PathBuf;

mod add;
mod del;
mod ls;
mod unzip;

macro_rules! common_args {
    (
        $(#[$meta: meta])*
        $name:ident {
            $($(#[$field_meta: meta])*
            $field_name:ident : $field_type:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        struct $name {
            file: PathBuf,
            #[arg(short)]
            password: Option<String>,
             $(
                $(#[$field_meta])*
                $field_name : $field_type
            ),*
        }
    };
}

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
    #[arg(num_args=1..)]
    files: Vec<PathBuf>,
    #[arg(short)]
    target: PathBuf,
    #[arg(short)]
    password: Option<String>,
}

#[derive(ClapArgs, Debug)]
struct LsArgs {
    file: PathBuf,
    #[arg(short, default_value_t = false)]
    recursive: bool,
    #[arg[short, num_args=0..]]
    path: Option<Vec<PathBuf>>,
}

common_args!(
    #[derive(ClapArgs, Debug)]
    UnzipArgs {
        output: Option<PathBuf>,
    }
);

#[derive(Subcommand, Debug)]
enum Commands {
    Unzip {
        file: PathBuf,
        #[arg(short)]
        password: Option<String>,
        #[arg(short)]
        output: Option<String>,
    },
    Add(DelOrAddArgs),
    Del(DelOrAddArgs),
    Ls(LsArgs),
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
                Commands::Ls(args) => {
                    let _ = ls::list(args);
                }
                Commands::Unzip {
                    file,
                    password,
                    output,
                } => {}
            }
        }
        None => {
            println!("No command provided.");
        }
    }
}
