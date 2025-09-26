use std::path::{Path, PathBuf};

pub fn find_script(module_dir: &Path, script_name: &str) -> Option<PathBuf> {
    let script_path = module_dir.join(script_name);
    
    // Check if the script exists and is a file
    if script_path.is_file() {
        Some(script_path)
    } else {
        None
    }
}
