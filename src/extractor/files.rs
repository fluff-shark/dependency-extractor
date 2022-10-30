use std::{path::{Path,PathBuf,StripPrefixError}, fs};
use globwalk::GlobWalkerBuilder;

const WORKSPACE_FOLDER: &str = ".dependencies";

// Delete the workspace where we'll put all our dependency files,
// and then return it.
pub fn clear_workspace(base_path: &Path) -> PathBuf {
    let workspace = base_path.join(WORKSPACE_FOLDER);
    let workspace_path = workspace.as_path();
    println!("Removing {}", workspace_path.display());
    _ = fs::remove_dir_all(workspace_path); // errors are ok here... the directory may not exist.
    return workspace;
}

pub struct FileToProcess {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

pub fn get_files_to_process(base_path: &Path, workspace: &Path) -> Vec<Result<FileToProcess, StripPrefixError>> {
    return find_files_to_process(base_path).iter().map(|input_file| -> Result<FileToProcess, StripPrefixError> {
        let output_file = get_dependency_file_name(base_path, workspace, input_file)?;
        return Ok(FileToProcess{
            input_file: input_file.to_owned(),
            output_file: output_file,
        })
    }).collect();
}


// Get the name of the file where we should log the dependency info.
fn get_dependency_file_name(base_path: &Path, workspace: &Path, input_file: &Path) -> Result<PathBuf, StripPrefixError> {
    let dependency_file_name = input_file.strip_prefix(base_path)?;
    println!("logging {} to {}", input_file.display(), workspace.join(dependency_file_name).display());
    return Ok(workspace.join(dependency_file_name));
}

fn find_files_to_process(base_path: &Path) -> Vec<PathBuf> {
    let walker = match GlobWalkerBuilder::from_patterns(&base_path, &["**/*.{js,jsx,ts,tsx}"]).build() {
        Ok(the_walker) => the_walker,
        Err(e) => panic!("Glob error: {}", e),
    };

    return walker.map(|m| match m {
        Ok(dir_entry) => dir_entry.into_path(),
        Err(e) => panic!("Error searching {}: {}", base_path.display(), e),
    }).collect();
}
