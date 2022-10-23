mod config;
mod files;

fn main() {
    let config = config::parse_config();
    let files = files::find_ts_filenames(config.base_path);
    dbg!(files);
}
