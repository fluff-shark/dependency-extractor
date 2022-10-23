mod config;
mod extractor;

fn main() {
    let config = config::parse_config();
    extractor::extract_dependndencies(config.base_path.as_path());
}
