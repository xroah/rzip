use clap::Parser;
use std::env;

mod unzip;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(version)]
struct Args {
    name: Option<Vec<String>>,
    
    #[command(subcommand)]
    command: Option<unzip::Commands>,
}

pub fn greet () {
    let args = Args::parse();

    match args.name {
        Some(names) => {
            for name in names {
                println!("Hello, {}!", name);
            }
        },
        None => {
            println!("Hello, World!");
        }
    }
    
    match args.command {
        Some(unzip::Commands::Unzip { name }) => {
            println!("Unzipping file: {}", name);
            // Here you would call your unzip function
        },
        None => {
            println!("No command provided.");
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        println!("Current directory: {:?}", current_dir)
    }
}