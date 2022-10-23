use std::path::Path;

pub fn log_dependencies(input: &Path, output: &Path) {
    println!("Logging {} to {}", input.display(), output.display());
}