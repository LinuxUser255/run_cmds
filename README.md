# run_cmds

A Rust CLI application that serves as a dispatcher for shell scripts. This program allows you to organize and execute shell scripts through a unified command-line interface.

## Overview

`run_cmds` is designed to manage and execute shell scripts stored in the `src/modules/` directory. Instead of running shell scripts directly, you use this Rust CLI to discover, validate, and execute them with proper error handling and exit code propagation.

## Architecture

```
src/
├── main.rs              # Entry point and command dispatcher
├── execution/
│   ├── mod.rs           # Module declarations and re-exports
│   └── runners.rs       # Script execution logic
├── utils/
│   ├── mod.rs           # Module declarations
│   └── paths.rs         # Script discovery and path utilities
└── modules/
    ├── first_module.sh  # Example shell script
    ├── second_module.sh # Example shell script
    └── third_module.sh  # Example shell script
```

### Component Details

- **`main.rs`**: The primary entry point that handles command-line arguments, provides usage information, and coordinates script execution
- **`execution/runners.rs`**: Contains the `run_script()` function that executes shell scripts using `bash` and handles process management
- **`utils/paths.rs`**: Implements `find_script()` to locate scripts in the modules directory
- **`src/modules/`**: Directory containing executable shell scripts that can be invoked by the CLI

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

### Basic Commands

```bash
# Show available scripts and usage information
cargo run

# Execute a specific shell script
cargo run -- <script_name>

# Examples:
cargo run -- first_module.sh
cargo run -- second_module.sh
cargo run -- third_module.sh
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

# Build and run in one step
cargo run -- <script_name>
```

## How It Works

1. **Argument Parsing**: The CLI accepts a script name as a command-line argument
2. **Script Discovery**: Uses the `find_script()` function to locate the requested script in `src/modules/`
3. **Validation**: Checks if the script file exists and is accessible
4. **Execution**: Runs the script using `bash` through Rust's `std::process::Command`
5. **Exit Code Propagation**: Returns the script's exit code to maintain proper shell behavior

### Example Workflow

```bash
$ cargo run -- first_module.sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/run_cmds first_module.sh`
this is the FIRST module
```

If a script doesn't exist:
```bash
$ cargo run -- nonexistent.sh
Script 'nonexistent.sh' not found in src/modules
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

3. Execute it through the CLI:
   ```bash
   cargo run -- my_new_script.sh
   ```

## Script Requirements

All shell scripts in `src/modules/` should:
- Have a proper shebang line: `#!/usr/bin/env bash`
- Be executable: `chmod +x script_name.sh`
- Follow standard bash scripting practices
- Exit with appropriate exit codes (0 for success, non-zero for failure)

## Error Handling

The CLI provides comprehensive error handling:
- **Missing arguments**: Shows usage information and lists available scripts
- **Script not found**: Clear error message indicating the script doesn't exist
- **Execution errors**: Propagates the script's exit code and displays execution errors
- **Permission errors**: Reports if scripts are not executable

## Development Notes

- The project uses Rust's module system to organize code into logical components
- Script execution is handled through `std::process::Command` for better control and error handling
- The CLI automatically discovers scripts in `src/modules/` and presents them to users
- Exit codes are properly propagated from executed scripts to maintain shell conventions

## Project Structure Benefits

- **Separation of Concerns**: Clear division between argument parsing, script discovery, and execution
- **Extensibility**: Easy to add new script management features
- **Error Safety**: Rust's type system prevents common runtime errors
- **Cross-platform**: Works on any system with Rust and Bash support

## Contributing

When adding new features or modifying existing code:
1. Ensure all tests pass: `cargo test`
2. Format code: `cargo fmt`
3. Check for lints: `cargo clippy`
4. Update documentation as needed
5. Test manually with various script scenarios