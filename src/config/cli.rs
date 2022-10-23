use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Glob pattern of files which should be analyzed
   #[arg(short, long)]
   pub files: Option<String>,
}

pub fn parse_cli_options() -> Args {
    return Args::parse();
}