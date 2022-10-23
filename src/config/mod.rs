mod cli;

#[derive(Debug)]
pub struct Config {
    // The glob of files which will be analyzed.
    pub file_glob: String
}

pub fn parse_config() -> Config {
    let args = cli::parse_cli_options();
    return Config {
        file_glob: args.files.unwrap_or(String::from("**/*.{js,jsx,ts,tsx}")),
    }
}