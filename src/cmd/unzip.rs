use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Unzip { name: String },
}
