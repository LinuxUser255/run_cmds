use std::env;
use std::path::Path;
use std::process;

mod execution;
mod utils;

use execution::run_script;
use utils::paths::find_script;

fn handle_run_module(module_dir: &Path, script_name: &str) -> i32 {
    match find_script(module_dir, script_name) {
        Some(script_path) => {
            match run_script(&script_path, "module", module_dir) {
                Ok(status) => {
                    if status.success() {
                        0
                    } else {
                        status.code().unwrap_or(1)
                    }
                }
                Err(e) => {
                    eprintln!("Error running script: {}", e);
                    1
                }
            }
        }
        None => {
            eprintln!("Script '{}' not found in {}", script_name, module_dir.display());
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <script_name>", args[0]);
        println!("Available scripts:");
        
        // Retrieve the directory where the executable is located
        let exe_path = env::current_exe().unwrap_or_else(|_| {
            Path::new(&args[0]).to_path_buf()
        });
        let exe_dir = exe_path.parent().unwrap();
        
        // Module directory is relative to the executable --  will be in src/modules in dev
        let module_dir = exe_dir.join("../src/modules").canonicalize()
            .unwrap_or_else(|_| Path::new("src/modules").to_path_buf());
        
        if let Ok(entries) = std::fs::read_dir(&module_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".sh") {
                            println!("  {}", name);
                        }
                    }
                }
            }
        }
        process::exit(1);
    }

    let script_name = &args[1];
    
    // Retrieve the directory where the executable is located
    let exe_path = env::current_exe().unwrap_or_else(|_| {
        Path::new(&args[0]).to_path_buf()
    });
    let exe_dir = exe_path.parent().unwrap();
    
    // Module directory is relative to the executable (in development, this will be in src/modules)
    let module_dir = exe_dir.join("../src/modules").canonicalize()
        .unwrap_or_else(|_| Path::new("src/modules").to_path_buf());

    let exit_code = handle_run_module(&module_dir, script_name);
    process::exit(exit_code);
}
