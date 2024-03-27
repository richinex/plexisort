use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
// organizer.rs
use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::processing_mode::ProcessingMode;

pub fn organize_file(source_path: &Path, destination_path: &Path, mode: &mut ProcessingMode) -> Result<(), io::Error> {
    match mode {
        ProcessingMode::DryRun(virtual_dir) => {
            let mut parts: Vec<String> = destination_path.iter().map(|s| s.to_string_lossy().to_string()).collect();
            if let Some(file_name) = source_path.file_name().and_then(|n| n.to_str()) {
                parts.push(file_name.to_string());
            }
            virtual_dir.add_path(&parts);
            Ok(())
        }
        ProcessingMode::Live => {
            fs::rename(source_path, destination_path).map_err(|e| {
                println!("Failed to move file from {} to {}: {}", source_path.display(), destination_path.display(), e);
                e
            })?;
            println!("Successfully moved file from {} to {}", source_path.display(), destination_path.display());
            log_move_operation(source_path, destination_path).map_err(|log_err| {
                eprintln!("Failed to log the move operation: {}", log_err);
                log_err
            })
        }
    }
}

#[cfg(not(feature = "test_env"))]
fn log_move_operation(original_path: &Path, destination_path: &Path) -> std::io::Result<()> {
    use std::io::Write;

    use serde_json::json;

    let log_entry = json!({
        "original_path": original_path.to_str(),
        "destination_path": destination_path.to_str(),
    });

    // Validate JSON format
    if let Ok(log_str) = serde_json::to_string(&log_entry) {
        let mut log_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("undo_log.jsonl")?;

        // Write the JSON string and manually append a newline
        if let Err(e) = write!(log_file, "{}\n", log_str) {
            return Err(e); // Handle write error
        }

        // Explicitly flush the buffer to ensure the newline is written
        log_file.flush()?;
    } else {
        eprintln!("Invalid JSON format for log entry");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid JSON format",
        ));
    }

    Ok(())
}

#[cfg(feature = "test_env")]
fn log_move_operation(_original_path: &Path, _destination_path: &Path) -> std::io::Result<()> {
    Ok(())
}


pub fn undo_last_actions() -> io::Result<()> {
    let log_path = Path::new("undo_log.jsonl");
    let file = File::open(log_path)?;
    let reader = BufReader::new(file);

    let mut affected_dirs = HashSet::new();

    // Process each line in the undo log
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let action: Value = serde_json::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let original_path_str = action["original_path"].as_str().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing 'original_path'"))?;
        let destination_path_str = action["destination_path"].as_str().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing 'destination_path'"))?;

        let original_path = Path::new(original_path_str);
        let destination_path = Path::new(destination_path_str);

        if destination_path.exists() {
            if let Some(parent_dir) = original_path.parent() {
                fs::create_dir_all(parent_dir)?;
            }

            fs::rename(&destination_path, &original_path)?;
            println!("Reversed move: {} -> {}", destination_path.display(), original_path.display());

            let mut current_dir = destination_path.parent();
            while let Some(dir) = current_dir {
                affected_dirs.insert(dir.to_path_buf());
                current_dir = dir.parent();
            }
        } else {
            return Err(Error::new(ErrorKind::NotFound, format!("Destination file does not exist: {}", destination_path.display())));
        }
    }

    // Attempt to remove the log file before removing directories
    if log_path.exists() {
        fs::remove_file(log_path)?;
        println!("Undo log cleared.");
    } else {
        println!("Undo log file not found or already removed.");
    }

    // Now attempt to remove directories
    remove_directories(affected_dirs)?;

    Ok(())
}

pub fn remove_directories(dirs: HashSet<PathBuf>) -> io::Result<()> {
    let mut dirs_to_remove: Vec<_> = dirs.into_iter().collect();
    dirs_to_remove.sort_by_key(|dir| dir.as_path().components().count());
    dirs_to_remove.reverse();

    for dir in dirs_to_remove {
        if dir.as_os_str().is_empty() {
            println!("Encountered an empty directory path, skipping.");
            continue;
        }

        println!("Checking if directory is empty: {}", dir.display());
        if is_dir_empty(&dir)? {
            println!("Removing directory: {}", dir.display());
            if let Err(e) = fs::remove_dir(&dir) {
                println!("Failed to remove directory {}: {}", dir.display(), e);
            } else {
                println!("Directory removed: {}", dir.display());
            }
        } else {
            println!("Directory not empty, skipping: {}", dir.display());
        }
    }

    Ok(())
}


fn is_dir_empty(dir: &Path) -> io::Result<bool> {
    let mut entries = fs::read_dir(dir)?;
    Ok(entries.next().is_none())
}


pub fn clear_undo_log() -> std::io::Result<()> {
    let log_path = "undo_log.jsonl";
    if Path::new(log_path).exists() {
        fs::remove_file(log_path)?;
        println!("Undo log cleared.");
    } else {
        println!("No undo log file found to clear."); 
    }
    Ok(())
}



pub fn print_current_structure(path: &Path, prefix: &str) {
    // Check if the path is a directory or a file
    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap_or_else(|err| {
            panic!("Failed to read directory {}: {}", path.display(), err);
        });

        let mut entries_vec: Vec<PathBuf> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
        // Sort the entries for a consistent output
        entries_vec.sort();

        for (i, entry) in entries_vec.iter().enumerate() {
            let file_name = entry.file_name().unwrap().to_str().unwrap();
            let connector = if i == entries_vec.len() - 1 { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, file_name);

            // If the entry is a directory, recursively print its contents
            if entry.is_dir() {
                let new_prefix = if i == entries_vec.len() - 1 {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                print_current_structure(entry, &new_prefix);
            }
        }
    } else {
        // If the path is a file, just print its name
        println!("{}── {}", prefix, path.file_name().unwrap().to_str().unwrap());
    }
}