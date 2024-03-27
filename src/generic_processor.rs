use crate::organizer::organize_file;
use crate::processing_mode::ProcessingMode;
use crate::traits::processor::Processor;
use std::fs;
use std::path::PathBuf;
use log::{debug, error};


pub struct GenericProcessor;

impl Processor for GenericProcessor {
    fn process(&self, path: &PathBuf, destination: &PathBuf, mode: &mut ProcessingMode) {
        // Define the destination directory based on the subfolder and ensure it exists
        let destination_dir = destination.join(self.get_destination_subfolder(path));

        // Specify the destination path for the file, correctly appending the filename
        let destination_path = destination_dir.join(path.file_name().unwrap());

        match mode {
            ProcessingMode::DryRun(virtual_directory) => {
                // In DryRun mode, simulate the action without making changes
                debug!("Would move {} to {}", path.display(), destination_path.display());
                let path_parts: Vec<String> = destination_path.iter().map(|s| s.to_string_lossy().to_string()).collect();
                virtual_directory.add_path(&path_parts);
            },
            ProcessingMode::Live => {
                // In Live mode, actually create the directory and move the file
                // Ensure the directory structure exists
                if let Err(e) = fs::create_dir_all(&destination_dir) {
                    error!("Error creating destination directory: {}", e);
                    return;
                }
                if let Err(e) = organize_file(path, &destination_path, mode) {
                    error!("Failed to organize file: {}", e);
                } else {
                    debug!("Successfully moved file from {} to {}", path.display(), destination_path.display());
                }
            }
        }
    }

    fn get_destination_subfolder(&self, _path: &PathBuf) -> PathBuf {
        // Adjust the returned subfolder name as needed
        PathBuf::from("Other_Files")
    }
}


#[cfg(test)]
mod generic_processor_tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_generic_processor_live() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let destination_dir = temp_dir.path().join("destination");
        fs::create_dir_all(&source_dir).unwrap();
        // Create a generic file; the type or content doesn't matter for this processor
        let generic_file_path = source_dir.join("generic_file.txt");
        let mut file = File::create(&generic_file_path).unwrap();
        writeln!(file, "Generic file content").unwrap();

        let processor = GenericProcessor {};
        let mut mode = ProcessingMode::Live;

        processor.process(&generic_file_path, &destination_dir, &mut mode);

        // The expected destination is within the "Other_Files" directory
        let expected_destination = destination_dir.join("Other_Files").join("generic_file.txt");
        assert!(expected_destination.exists(), "Generic file was not moved to the correct destination in Live mode.");
    }
}
