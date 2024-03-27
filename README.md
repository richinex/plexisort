
# Plexisort

Plexisort is a command-line tool designed to organize your files based on metadata. It allows for flexible source and destination directory settings, supports dry-run operations for safe previews of potential changes, and even offers an undo functionality for reversing the last set of file movements.

## Version
1.0

## Author
Richard Chukwu <richinex@gmail.com>

## Features
- **Custom Config File**: Use a custom configuration file to specify operational parameters.
- **Source Directory**: Set one or more source directories for the organization process.
- **Destination Directory**: Define a specific destination directory for organized files.
- **Dry Run**: Execute the tool in a mode that shows what would be done without making any changes.
- **Undo**: Revert the last set of changes made by the tool.

## How to Use
1. **Installation**: Ensure you have Rust installed on your system. Clone this repository and build the project using `cargo build --release`.
2. **Running**: Execute the tool with `cargo run -- [OPTIONS]`. The following options are available:
   - `-c, --config <FILE>`: Sets a custom config file.
   - `--source <SOURCE_DIR>`: Sets the source directory(s). Multiple directories can be specified.
   - `--destination <DEST_DIR>`: Sets the destination directory.
   - `--dry-run`: Runs the organizer without making any changes.
   - `--undo`: Reverts the last set of file movements.

## Building the Configuration
If not using a configuration file, the tool requires at least the source and destination directories to be specified through command-line options.

## Config File Example
You can use a `config.toml` file like this for the configuration option:

```toml
source_directories = ["path/to/cluttered_data"]
destination = "organized_data/organized"
```

## Example Command using the config.toml file
```bash
cargo run -- --config config.toml
```


## Example Command using source and destination paths
```bash
cargo run -- --source /path/to/source --destination /path/to/destination
```

This will organize files from `/path/to/source` to `/path/to/destination` based on their metadata.

## Logging
The application provides informative logging during its operation, indicating the progress and actions taken or to be taken in dry-run mode.

## Contributions
Contributions are welcome! Please feel free to submit pull requests or create issues for bugs and feature requests.

## License
This project is open-source and available under the MIT License.
