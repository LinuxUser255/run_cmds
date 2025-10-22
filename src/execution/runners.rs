// Use this module to execute other shell scripts
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
// This dir:  src/modules/

pub fn find_script(module_dir: &Path, script_name: &str) -> Option<PathBuf> {
    let script_path = module_dir.join(script_name);

    // Check if the script exists and is a file
    if script_path.exists() {
        Some(script_path)
    } else {
        None
    }
}

pub fn run_script(
    script_path: &Path,
    _script_type: &str,
    _module_dir: &Path,
) -> io::Result<ExitStatus> {
    let status = Command::new("bash").arg(script_path).status()?;

    Ok(status)
}
