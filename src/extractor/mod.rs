use std::path::Path;

mod dependencies;
mod files;

pub fn extract_dependndencies(base_path: &Path) {
    let workspace = files::clear_workspace(base_path);
    let files_to_process = files::get_files_to_process(base_path, workspace.as_path());
    files_to_process.iter().for_each(|file| match file {
        Ok(file) => dependencies::log_dependencies(&file.input_file, &file.output_file),
        Err(e) => println!("{}", e),
    });
}
