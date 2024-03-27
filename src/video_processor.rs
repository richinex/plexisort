use crate::organizer::organize_file;
use crate::processing_mode::ProcessingMode;
use crate::traits::processor::Processor;
use std::fs;
use std::path::PathBuf;

pub struct VideoProcessor;

impl Processor for VideoProcessor {
    fn process(&self, path: &PathBuf, destination: &PathBuf, mode: &mut ProcessingMode) {
        // Determine the destination directory without including the filename
        let destination_dir = destination.join(self.get_destination_subfolder(path));



        // Now the destination path includes the filename correctly
        let destination_path = destination_dir.join(path.file_name().unwrap());

        match mode {
            ProcessingMode::DryRun(virtual_directory) => {
                // In DryRun mode, simulate the action
                println!("Would move {} to {}", path.display(), destination_path.display());
                let path_parts: Vec<String> = destination_path.iter().map(|s| s.to_string_lossy().to_string()).collect();
                virtual_directory.add_path(&path_parts);
            },
            ProcessingMode::Live => {
                // In Live mode, actually create the directory and move the file
                // Ensure the directory structure exists
                if let Err(e) = fs::create_dir_all(&destination_dir) {
                    println!("Error creating destination directory: {}", e);
                    return;
                }
                if let Err(e) = organize_file(path, &destination_path, mode) {
                    println!("Failed to organize file: {}", e);
                } else {
                    println!("Successfully moved file from {} to {}", path.display(), destination_path.display());
                }
            }
        }
    }

    fn get_destination_subfolder(&self, _path: &PathBuf) -> PathBuf {
        PathBuf::from("Videos")
    }
}


#[cfg(test)]
mod video_processor_tests {


    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_video_processor_live() {
        let processor = VideoProcessor {};
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let destination_dir = temp_dir.path().join("destination");
        fs::create_dir_all(&source_dir).unwrap();
        let video_file_path = source_dir.join("test_video.mp4");
        let mut file = File::create(&video_file_path).unwrap();
        writeln!(file, "Dummy video content").unwrap();

        let mut mode = ProcessingMode::Live;

        processor.process(&video_file_path, &destination_dir, &mut mode);

        let expected_destination = destination_dir.join("Videos").join("test_video.mp4");
        print!("{}", expected_destination.display());

        assert!(expected_destination.exists(), "Document was not moved to the correct destination in Live mode.");
    }
}
