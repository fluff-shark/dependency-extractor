mod config;
mod extractor;

fn main() {
    let config = config::parse_config();
    match config.base_path.canonicalize() {
        Ok(path) => extractor::extract_dependndencies(path),
        Err(e) => panic!("Invalid path: {}", e),
    }
}
