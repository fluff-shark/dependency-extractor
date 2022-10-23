use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Path to the project root. The filse glob will be evaluated
   /// in the context of this path.
   #[arg(short, long)]
   pub base_path: Option<String>,
}

pub fn parse_cli_options() -> Args {
    return Args::parse();
}