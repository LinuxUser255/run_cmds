# run_cmds

An interactive Rust CLI application that serves as a menu-driven dispatcher for shell scripts. This program provides an intuitive interface to organize, discover, and execute shell scripts.

## Overview

`run_cmds` manages and executes shell scripts stored in the `src/modules/` directory through an interactive menu system. When launched without arguments, it presents a clean, formatted menu allowing you to select and run scripts with proper error handling and exit code propagation.

## Architecture

```
src/
├── main.rs              # Single file containing all application logic
└── modules/             # Shell scripts directory
    ├── first_module.sh  
    ├── second_module.sh 
    └── third_module.sh  
```

### Simplified Design

- **`main.rs`**: A single, self-contained file that handles:
  - Interactive menu system with dynamic script discovery
  - Command-line argument processing
  - Script execution with proper error handling
  - All helper functions for path resolution and script running
- **`src/modules/`**: Directory containing executable shell scripts accessible through the menu

## Installation & Setup

### Prerequisites

- Rust (latest stable version)
- Bash (for script execution)

### Building

```bash
# Clone or navigate to the project directory
cd /path/to/run_cmds

# Build the project
cargo build

# Or build for release
cargo build --release
```

## Usage

### Interactive Mode (Default)

Launch the program without arguments to enter the interactive menu:

```bash
# Launch interactive menu
cargo run

# Or if installed
./target/debug/run_cmds
```

This displays:

```
═══════════════════════════════════════════════════════════════════════════════

▶ OPTIONS:

  a) first_module
  b) second_module
  c) third_module
  q) quit | exit

═══════════════════════════════════════════════════════════════════════════════

Select an option: 
```

Simply type a letter (a, b, c) to run the corresponding script, or 'q' to quit. The menu reappears after each script execution.

### Direct Execution Mode

Bypass the menu and run scripts directly:

```bash
# Execute specific scripts
cargo run -- first_module.sh
cargo run -- second_module.sh

# Show help
cargo run -- --help

# Show about
cargo run -- --about
```




### Development Commands

```bash
# Run tests
cargo test

# Check code formatting
cargo fmt -- --check

# Apply code formatting
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

```

## How It Works

### Interactive Mode Workflow

1. **Launch**: Run without arguments to enter interactive mode
2. **Menu Display**: Automatically scans `src/modules/` for `.sh` files
3. **Dynamic Mapping**: Scripts are mapped to letters (a, b, c, etc.) alphabetically
4. **Selection**: User types the corresponding letter
5. **Execution**: Selected script runs with real-time output
6. **Feedback**: Shows "Script completed successfully!" or error message
7. **Loop**: Menu reappears for additional selections
8. **Exit**: Type 'q', 'quit', or 'exit' to terminate

### Direct Execution Workflow

1. **Argument Parsing**: Accepts script name as command-line argument
2. **Script Path**: Constructs path by joining module directory with script name
3. **Validation**: Verifies script exists in `src/modules/`
4. **Execution**: Runs script using `bash` via `std::process::Command`
5. **Exit Code**: Propagates script's exit code for shell compatibility

### Example Sessions

**Interactive Mode:**
```bash
$ cargo run

═══════════════════════════════════════════════════════════════════════════════

▶ OPTIONS:

  a) first_module
  b) second_module
  c) third_module
  q) quit | exit

═══════════════════════════════════════════════════════════════════════════════

Select an option: a

Running first_module.sh...

this is the FIRST module

Script completed successfully!

[Menu appears again]
```

**Direct Execution:**
```bash
$ cargo run -- first_module.sh
this is the FIRST module
```

## Adding New Scripts

1. Create a new shell script in `src/modules/`:
   ```bash
   touch src/modules/my_new_script.sh
   chmod +x src/modules/my_new_script.sh
   ```

2. Add your shell script content:
   ```bash
   #!/usr/bin/env bash
   echo "Hello from my new script!"
   # Add your commands here
   ```

3. The script automatically appears in the menu:
   ```
   ▶ OPTIONS:
   
     a) first_module
     b) my_new_script    # New script appears alphabetically
     c) second_module
     d) third_module
     q) quit | exit
   ```

## Script Requirements

All shell scripts in `src/modules/` should:
- Have a proper shebang line: `#!/usr/bin/env bash`
- Be executable: `chmod +x script_name.sh`
- Follow standard bash scripting practices
- Exit with appropriate exit codes (0 for success, non-zero for failure)

## Features

- **Interactive Menu**: User-friendly interface with visual formatting
- **Dynamic Discovery**: Automatically detects all `.sh` files
- **Alphabetical Mapping**: Scripts mapped to letters for easy selection
- **Continuous Operation**: Menu loop for running multiple scripts
- **Direct Execution**: Command-line argument support to bypass menu
- **Clean Output**: Well-formatted display with separators and status messages
- **Error Recovery**: Invalid selections don't crash the program
- **Exit Code Handling**: Proper propagation of script exit codes

## Error Handling

The CLI provides comprehensive error handling:
- **Interactive Mode**: Gracefully handles invalid menu selections
- **Script Not Found**: Clear error message with path information
- **Execution Errors**: Displays error details and exit codes
- **Permission Errors**: Reports if scripts are not executable
- **Empty Directory**: Notifies when no scripts are available

## Development Notes

- **Simplified Architecture**: All logic consolidated into a single `main.rs` file (~150 lines)
- **Zero Dependencies**: Uses only Rust standard library, no external crates needed
- **Dynamic Discovery**: Scripts found at runtime by scanning `src/modules/` directory
- **Clean Code**: Removed unnecessary abstractions and module hierarchies
- **Interactive Menu**: Implemented with `std::io` for user input
- **Alphabetical Sorting**: Scripts sorted and mapped to letters for easy selection
- **Extension Handling**: `.sh` extension removed in menu display for cleaner presentation
- **Continuous Loop**: Menu persists until explicit exit command

## Project Benefits

- **Simplicity**: Single file architecture makes the codebase easy to understand and modify
- **User Experience**: Intuitive menu interface requires no memorization
- **Minimal Dependencies**: No external crates means faster compilation and fewer security concerns
- **Maintainability**: All logic in one place reduces cognitive load and debugging time
- **Extensibility**: Easy to add new features without navigating complex module structures
- **Error Safety**: Rust's type system prevents runtime errors
- **Cross-platform**: Works on any system with Rust and Bash support
- **Zero Configuration**: Works immediately after compilation

## Contributing

When adding new features or modifying existing code:
1. Ensure all tests pass: `cargo test`
2. Format code: `cargo fmt`
3. Check for lints: `cargo clippy`
4. Update documentation as needed
5. Test both interactive and direct execution modes
