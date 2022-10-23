use std::path::PathBuf;

use globwalk::GlobWalkerBuilder;

pub fn find_ts_filenames(base_path: String) -> Vec<PathBuf> {
    let walker = match GlobWalkerBuilder::from_patterns(&base_path, &["**/*.{js,jsx,ts,tsx}"]).build() {
        Ok(the_walker) => the_walker,
        Err(e) => panic!("Glob error: {}", e),
    };

    return walker.map(|m| match m {
        Ok(dir_entry) => dir_entry.into_path(),
        Err(e) => panic!("Error searching {}: {}", base_path, e),
    }).collect();
}