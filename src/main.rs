
use std::env;
use crate::execution::runners::run_script;
use std::path::Path;
use std::process;

mod config;
mod execution;
// mod utils; // Removed duplicate module declaration



// Helper function to get the directories for the executable and the module directory
fn get_directories(program_name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let exe_path = env::current_exe().unwrap_or_else(|_| {
        Path::new(program_name).to_path_buf()
    });
    let exe_dir = exe_path.parent().unwrap().to_path_buf();
    let module_dir = exe_dir.join("../src/modules").canonicalize()
        .unwrap_or_else(|_| Path::new("src/modules").to_path_buf());

    (exe_dir, module_dir)
}

fn print_help_menu() {
    use crate::config::NAME;
    let module_dir = Path::new("src/modules");

    println!(
        r#"
Usage: {} [script_name]
═══════════════════════════════════════════════════════════════════════════════

▶ GENERAL OPTIONS:
  -a, --about          Show information about {}
  -h, --help           Show help information

═══════════════════════════════════════════════════════════════════════════════
"#, NAME, NAME
    );

    // Retrieve & display available modules
    let mut modules = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&module_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".sh") {
                        modules.push(name.to_string());
                    }
                }
            }
        }
    }

    println!("▶ AVAILABLE MODULES ({} found):", modules.len());
    if modules.is_empty() {
        println!("    (no modules found)");
    } else {
        for module in &modules {
            println!("    • {}", module);
        }
    }
    println!();
}

fn print_about() {
    use crate::config::NAME;
    println!("{} - A simple script runner for shell scripts", NAME);
}

fn print_usage_and_exit(program_name: &str) {
    let (_, module_dir) = get_directories(program_name); // ✅ Reuse existing function!

    eprintln!("Usage: {} <script_name>", program_name);
    eprintln!("Available scripts:");

    if let Ok(entries) = std::fs::read_dir(&module_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".sh") {
                        eprintln!("  {}", name);
                    }
                }
            }
        }
    }
    process::exit(1);
}

// Handle running the modules/shell scripts
fn handle_run_modules(module_dir: &Path, script_name: &str) -> i32 {
    match find_script(module_dir, script_name) {
        Some(script_path) => {
            // script_path is PathBuf (owned), convert to &Path for run_script
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
        print_usage_and_exit(&args[0]);
    }

    let (_, module_dir) = get_directories(&args[0]);

    let exit_code = match args[1].as_str() {
        "--help" | "-h" => {
            print_help_menu();
            0
        }
        "--about" | "-a" => {
            print_about();
            0
        }
        script_name => handle_run_modules(&module_dir, script_name)
    };

    process::exit(exit_code);
}

