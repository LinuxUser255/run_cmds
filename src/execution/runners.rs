// Use this module to execute other shell scripts
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::io;

pub fn run_script(script_path: &Path, _script_type: &str, _module_dir: &Path) -> io::Result<ExitStatus> {
    let status = Command::new("bash")
        .arg(script_path)
        .status()?;
    
    Ok(status)
}
