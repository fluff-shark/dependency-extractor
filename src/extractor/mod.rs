use std::path::PathBuf;

mod dependencies;
mod files;

pub fn extract_dependndencies(base_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let workspace = files::clear_workspace(&base_path);
    let files_to_process: Vec<files::FileToProcess> =
        files::get_files_to_process(&base_path, workspace.as_path())?;
    
    return files_to_process.iter()
      .map(|file| dependencies::log_dependencies(&base_path, &file.input_file, &file.output_file))
      .collect();
}
