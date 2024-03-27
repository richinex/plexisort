mod config;
mod document_processor;
mod file_processor; // Ensure this module is correctly defined and accessible
mod image_processor;
mod video_processor;
mod metadata;
mod organizer;
mod processing_mode;
mod virtual_directory;
mod compressed_file_processor;
mod generic_processor;
mod cli;
mod traits;

use config::Config;
use file_processor::process_directory;
use log::LevelFilter;
use processing_mode::ProcessingMode;

use simplelog::SimpleLogger;
use virtual_directory::VirtualDirectory;
use std::path::{Path, PathBuf};

use std::{fs, process};

use organizer::undo_last_actions;
use crate::organizer::{clear_undo_log, print_current_structure};
use crate::traits::DefaultProcessorFactory;

fn main() {
    init_logging();
    log::info!("Application starting up");
    let matches = cli::build_cli().get_matches();

    if let Err(e) = run_app(&matches) {
        // This is where the error gets logged, providing a single, clear error message.
        log::error!("Application error: {}", e);
        process::exit(1);
    } else {
        log::info!("Application ran successfully.");
    }
}


fn init_logging() {
    SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()).expect("Failed to initialize logging");
}

fn run_app(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_or_build_config(matches)?;
    let mut mode = determine_processing_mode(matches.contains_id("dry-run"));

    // Create an instance of the default processor factory
    let factory = DefaultProcessorFactory;

    println!("Original Directory Structure:");
    for source_directory in &config.source_directories {
        println!("\nDirectory: {}", source_directory);
        let path = Path::new(source_directory);
        print_current_structure(path, "");
    }

    check_source_directories(&config)?;

    // Now pass the factory when processing directories
    for source_directory in &config.source_directories {
        let source_path = PathBuf::from(source_directory);
        let dest_path = PathBuf::from(&config.destination);
        println!("Processing '{}'", source_path.display());
        process_directory(&source_path, &dest_path, &mut mode, &factory); // Adjusted to include factory
    }

    handle_undo(matches)?;

    Ok(())
}


fn check_source_directories(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    for source_directory in &config.source_directories {
        let source_path = Path::new(source_directory);
        if !source_path.exists() {
            // Removed log::error! and directly return Err
            return Err(format!("Error: Source directory '{}' does not exist.", source_directory).into());
        }
        if !source_path.is_dir() {
            return Err(format!("Error: '{}' is not a directory.", source_directory).into());
        }
        if let Err(e) = fs::read_dir(source_path) {
            return Err(format!("Error: No permission to read source directory '{}': {}", source_directory, e).into());
        }
    }
    Ok(())
}


fn handle_undo(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if matches.contains_id("undo") {
        match undo_last_actions() {
            Ok(_) => {
                println!("Undo actions completed successfully.");
                clear_undo_log().map_err(|e| format!("Failed to clear undo log: {}", e))?;
            }
            Err(e) => {
                return Err(format!("Error undoing actions: {}", e).into());
            }
        }
    }
    Ok(())
}

// Load or build config based on CLI arguments or config file
fn load_or_build_config(matches: &clap::ArgMatches) -> Result<Config, Box<dyn std::error::Error>> {
    if let Some(config_path) = matches.get_one::<String>("config") {
        Config::from_file(config_path)
    } else {
        build_config_from_cli_args(matches)
    }
}

// Build Config from CLI arguments
fn build_config_from_cli_args(matches: &clap::ArgMatches) -> Result<Config, Box<dyn std::error::Error>> {
    let source_directories: Vec<String> = matches.get_many::<String>("source")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    let destination = matches.get_one::<String>("destination")
        .expect("Destination directory is required")
        .clone();

    Ok(Config {
        source_directories,
        destination,
    })
}

// Determine the processing mode based on CLI arguments
fn determine_processing_mode(dry_run: bool) -> ProcessingMode {
    if dry_run {
        ProcessingMode::DryRun(VirtualDirectory::default())
    } else {
        ProcessingMode::Live
    }
}

