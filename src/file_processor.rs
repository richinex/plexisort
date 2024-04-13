use crate::traits::ProcessorFactory;

use crate::processing_mode::ProcessingMode;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;



pub fn process_directory(
    directory: &Path,
    base_dest: &PathBuf,
    mode: &mut ProcessingMode,
    factory: &dyn ProcessorFactory
) {
    let paths: Vec<PathBuf> = WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();

    paths.iter().for_each(|path| {
        let processor = factory.create_processor(path); // Use the factory
        processor.process(path, base_dest, mode);
    });

    // Debugging or DryRun mode output
    if let ProcessingMode::DryRun(virtual_dir) = mode {
        println!("Dry run: Preview of directory structure");
        virtual_dir.print_tree();
    }
}


#[cfg(test)]
mod tests {

    use crate::traits::{ProcessorFactory, TestProcessorFactory};
    use crate::virtual_directory::VirtualDirectory;


    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::Mutex;
    use tempfile::tempdir;


    #[test]
    fn test_virtual_directory_contains_file() {
        let temp_dir = tempdir().unwrap();

        // Ensure the source directory is created before creating a file within it
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap(); // Use create_dir_all to ensure the entire path is created

        let sample_file_path = source_dir.join("sample.txt");
        let mut file = File::create(&sample_file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let mut virtual_dir = VirtualDirectory::default();

        let dest_dir = "dest";
        let file_name = "sample.txt";

        // Convert the path to Vec<String> for the virtual directory
        let path_components = vec![dest_dir.to_string(), file_name.to_string()];
        virtual_dir.add_path(&path_components);

        // Ensure the file is now "contained" within the virtual directory
        assert!(virtual_dir.contains_file(&path_components));
    }


    #[test]
    fn test_processor_selection() {
        let factory = TestProcessorFactory {
            last_processor_type: Mutex::new(None),
        };
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();

        // Create a mock file for each processor type you want to test
        let document_file = source_dir.join("test.docx");
        let image_file = source_dir.join("test.png");
        // Add other file types as needed

        // Document Processor test
        factory.create_processor(&document_file);
        assert_eq!(*factory.last_processor_type.lock().unwrap(), Some("DocumentProcessor".to_string()), "DocumentProcessor was not selected for .docx files.");

        // Image Processor test
        factory.create_processor(&image_file);
        assert_eq!(*factory.last_processor_type.lock().unwrap(), Some("ImageProcessor".to_string()), "ImageProcessor was not selected for .png files.");

        // Add tests for other processor types as needed
    }
}


