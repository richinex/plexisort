use crate::organizer::organize_file;
use crate::processing_mode::ProcessingMode;
use crate::traits::processor::Processor;
use std::fs;
use std::path::PathBuf;
use log::{debug, error};



pub struct DocumentProcessor;

impl Processor for DocumentProcessor {
    fn process(&self, path: &PathBuf, destination: &PathBuf, mode: &mut ProcessingMode) {
        let file_extension = path.extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("")
            .to_lowercase();

        let destination_dir = self.determine_destination_dir(&file_extension, destination);
        let destination_path = destination_dir.join(path.file_name().unwrap());

        match mode {
            ProcessingMode::DryRun(virtual_directory) => {
                // In DryRun mode, just simulate the action
                debug!("Would move {} to {}", path.display(), destination_path.display());
                let path_parts: Vec<String> = destination_path.iter().map(|s| s.to_string_lossy().to_string()).collect();
                virtual_directory.add_path(&path_parts);


            },
            ProcessingMode::Live => {
                // In Live mode, actually create the directory and move the file
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
        PathBuf::new()
    }
}

impl DocumentProcessor {
    /// Determine the destination directory based on the file extension.
    fn determine_destination_dir(&self, extension: &str, base_dest: &PathBuf) -> PathBuf {
        let subfolder = match extension {
            "doc" | "docx" => "Word_Documents",
            "xls" | "xlsx" => "Excel_Spreadsheets",
            "ppt" | "pptx" => "PowerPoint_Presentations",
            "csv"  => "CSV_Files",
            "json" | "yaml" | "yml" => "Config_Files",
            "pdf" => "PDFs",
            "html" => "Web_Pages",
            "txt" => "Text_Files",
            _ => "Uncategorized_Documents",
        };
        base_dest.join("Documents").join(subfolder)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_document_processor_live() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let destination_dir = temp_dir.path().join("destination");
        fs::create_dir_all(&source_dir).unwrap();
        let document_file_path = source_dir.join("test_document.txt");
        let mut file = File::create(&document_file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        let processor = DocumentProcessor {};
        let mut mode = ProcessingMode::Live;

        processor.process(&document_file_path, &destination_dir, &mut mode);

        // Update the expected destination to include "Documents"
        let expected_destination = destination_dir.join("Documents").join("Text_Files").join("test_document.txt");
        assert!(expected_destination.exists(), "Document was not moved to the correct destination in Live mode.");
    }


    #[test]
    fn test_document_processor_logic() {
        let processor = DocumentProcessor {};
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let destination_dir = temp_dir.path().join("destination");
        fs::create_dir_all(&source_dir).unwrap();
        let document_file_path = source_dir.join("test_document.txt");
        let mut file = File::create(&document_file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        let mut mode = ProcessingMode::Live;

        processor.process(&document_file_path, &destination_dir, &mut mode);

        // Update the expected destination to include "Documents"
        let expected_destination = destination_dir.join("Documents").join("Text_Files").join("test_document.txt");

        assert!(expected_destination.exists(), "Document was not moved to the correct destination in Live mode.");
    }

}