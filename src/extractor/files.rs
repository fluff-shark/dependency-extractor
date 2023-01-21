use std::{path::{Path,PathBuf}, fs};
use globwalk::GlobWalkerBuilder;

const WORKSPACE_FOLDER: &str = ".dependencies";

// Delete the workspace where we'll put all our dependency files,
// and then return it.
pub fn clear_workspace(base_path: &Path) -> PathBuf {
    let workspace = base_path.join(WORKSPACE_FOLDER);
    let workspace_path = workspace.as_path();
     // errors are ok here... the directory may not exist.
    _ = fs::remove_dir_all(workspace_path);
    return workspace;
}

pub struct FileToProcess {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

pub fn get_files_to_process(base_path: &Path, workspace: &Path) -> Result<Vec<FileToProcess>, Box<dyn std::error::Error>> {
    let files_to_process = find_files_to_process(base_path)?;
    return files_to_process.iter().map(|input_file| -> Result<FileToProcess, Box<dyn std::error::Error>> {
        let output_file = get_dependency_file_name(base_path, workspace, input_file)?;
        return Ok(FileToProcess{
            input_file: input_file.to_owned(),
            output_file: output_file,
        })
    }).collect();
}


// Get the name of the file where we should log the dependency info.
fn get_dependency_file_name(base_path: &Path, workspace: &Path, input_file: &Path)
    -> Result<PathBuf, Box<dyn std::error::Error>>
{
    let old_extension = input_file.extension().unwrap().to_str().unwrap();
    let new_extension = format!("{}.yaml", old_extension);
    let dependency_file_name = input_file.strip_prefix(base_path)?.with_extension(new_extension);
    return Ok(workspace.join(dependency_file_name));
}

fn find_files_to_process(base_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let walker =
        GlobWalkerBuilder::from_patterns(&base_path, &["**/*.{js,jsx,ts,tsx}"]).build()?;
    return walker.map(|maybe_dir_entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
        let dir_entry = maybe_dir_entry?;
        return Ok(dir_entry.into_path());
    }).collect();
}
