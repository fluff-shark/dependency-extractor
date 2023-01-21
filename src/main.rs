mod config;
mod extractor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::parse_config();
    let base_path = config.base_path.canonicalize()?;
    return extractor::extract_dependndencies(base_path)
}
