use clap::{Args as ClapArgs, Parser, Subcommand};
use std::{error::Error, path::PathBuf};

mod add;
mod del;
mod ls;
mod unzip;
mod zip;

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
            #[arg(short, long)]
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
    #[arg(short, long)]
    password: Option<String>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(
        long,
        help = "Include the directory itself when compressing a directory",
        default_value_t = false
    )]
    dir: bool,
    #[arg(short = 'w', long, default_value_t = false)]
    overwrite: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(ClapArgs, Debug)]
struct DelOrAddArgs {
    #[arg(num_args=1..)]
    files: Vec<PathBuf>,
    #[arg(short, long)]
    target: PathBuf,
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(ClapArgs, Debug)]
struct LsArgs {
    file: PathBuf,
    #[arg(short, long, default_value_t = false)]
    recursive: bool,
    #[arg[short, long, num_args=0..]]
    path: Option<Vec<PathBuf>>,
}

common_args!(
    #[derive(ClapArgs, Debug)]
    UnzipArgs {
        #[arg(short, long)]
        output: Option<PathBuf>,
    }
);

#[derive(Subcommand, Debug)]
enum Commands {
    Unzip(UnzipArgs),
    Add(DelOrAddArgs),
    Del(DelOrAddArgs),
    Ls(LsArgs),
}

pub fn create_cmd() {
    let args = Args::parse();
    let mut ret: Result<(), Box<dyn Error>> = Ok(());

    match args.command {
        Some(cmd) => {
            println!("Command: {:?}", cmd);

            match cmd {
                Commands::Add(args) => {}
                Commands::Del(args) => {
                    ret = del::delete(args);
                }
                Commands::Ls(args) => {
                    ret = ls::list(args);
                }
                Commands::Unzip(args) => {
                    ret = unzip::extract(args);
                }
            }
        }
        None => {
            ret = zip::compress(args);
        }
    }

    if let Err(err) = ret {
        println!("{err:?}");
    }
}
