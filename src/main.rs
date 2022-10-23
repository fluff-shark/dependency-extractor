mod config;

fn main() {
    let config = config::parse_config();
    dbg!(config);
}
