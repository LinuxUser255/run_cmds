use std::env;
use std::io::{self, Write};
use crate::execution::runners::{run_script, find_script};
use std::path::Path;
use std::process;

mod config;
mod execution;
// Shell scripts located in src/modules/

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

fn print_menu(modules: &[String]) {
    println!("\n═══════════════════════════════════════════════════════════════════════════════\n");
    println!("▶ OPTIONS:\n");

    for (i, module) in modules.iter().enumerate() {
        let letter = (b'a' + i as u8) as char;
        let module_name = module.trim_end_matches(".sh");
        println!("  {}) {}", letter, module_name);
    }
    println!("  q) quit | exit\n");
    println!("═══════════════════════════════════════════════════════════════════════════════\n");
}

fn get_available_modules(module_dir: &Path) -> Vec<String> {
    let mut modules = Vec::new();
    if let Ok(entries) = std::fs::read_dir(module_dir) {
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
    modules.sort();
    modules
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
    let (_, module_dir) = get_directories(&args[0]);

    // If arguments provided, handle them
    if args.len() >= 2 {
        let exit_code = match args[1].as_str() {
            "--help" | "-h" => {
                print_usage_and_exit(&args[0]);
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

    // No arguments - show interactive menu
    let modules = get_available_modules(&module_dir);

    if modules.is_empty() {
        eprintln!("No modules found in {}", module_dir.display());
        process::exit(1);
    }

    loop {
        print_menu(&modules);
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();

        if choice == "q" || choice == "quit" || choice == "exit" {
            println!("Exiting...");
            process::exit(0);
        }

        // Check if it's a letter option (a, b, c, etc.)
        if choice.len() == 1 {
            if let Some(first_char) = choice.chars().next() {
                if first_char.is_ascii_lowercase() {
                    let index = (first_char as usize) - ('a' as usize);
                    if index < modules.len() {
                        let script_name = &modules[index];
                        println!("\nRunning {}...\n", script_name);
                        let exit_code = handle_run_modules(&module_dir, script_name);
                        if exit_code == 0 {
                            println!("\nScript completed successfully!");
                        }
                        continue;
                    }
                }
            }
        }

        println!("Invalid option. Please try again.");
    }
}

