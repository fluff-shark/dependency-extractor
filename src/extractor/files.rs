use std::{path::{Path,PathBuf}, fs};
use globwalk::GlobWalkerBuilder;

const WORKSPACE_FOLDER: &str = ".dependencies";

// Delete the workspace where we'll put all our dependency files,
// and then return it.
pub fn clear_workspace(base_path: &Path) -> PathBuf {
    let workspace = base_path.join(WORKSPACE_FOLDER);
    let workspace_path = workspace.as_path();
    println!("Removing {}", workspace_path.display());
    fs::remove_dir_all(workspace_path); // errors are ok here... directory may not exist.
    return workspace;
}

pub struct FileToProcess {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

pub fn get_files_to_process(base_path: &Path, workspace: &Path) -> Vec<FileToProcess> {
    return find_files_to_process(base_path).iter().map(|input_file| FileToProcess {
        input_file: input_file.to_owned(),
        output_file: get_dependency_file_name(base_path, workspace, input_file),
    }).collect();
}


// Get the name of the file where we should log the dependency info.
fn get_dependency_file_name(base_path: &Path, workspace: &Path, input_file: &Path) -> PathBuf {
    let dependency_file_name = match input_file.strip_prefix(base_path) {
        Ok(file) => file,
        Err(e) => panic!("Error removing prefix {} from file {}: {}", base_path.display(), input_file.display(), e),
    };
    println!("logging {} to {}", input_file.display(), workspace.join(dependency_file_name).display());
    return workspace.join(dependency_file_name);
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
