use std::path::PathBuf;

use glob::glob;

pub fn find_ts_filenames(base_path: String) -> Vec<PathBuf> {
    let full_glob = format!("{}/**/*jsx", base_path);

    return match glob(&full_glob) {
        Ok(paths) => paths.map(|glob_result| match glob_result {
            Ok(path_buf) => path_buf,
            Err(e) => panic!("Error resolving paths against glob: {}", e),
        }),
        Err(pattern_error) => panic!("Error compiling glob {}: {}", full_glob, pattern_error),
    }.collect();
}