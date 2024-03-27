use std::path::{Path, PathBuf};

use crate::virtual_directory::VirtualDirectory;

pub enum ProcessingMode {
    DryRun(VirtualDirectory),
    Live,
}


impl ProcessingMode {
    /// Checks if a file exists within the processing mode's context. For `DryRun`, it checks
    /// within the virtual directory. For `Live`, it always returns `false` as no virtual structure
    /// is inspected.
    ///
    /// # Parameters
    /// - `file_path`: The path of the file to check.
    /// - `dest_path`: The destination path to consider for the check.
    ///
    /// # Returns
    /// - `true` if the file exists within the virtual directory (`DryRun` mode).
    /// - `false` otherwise, or always in `Live` mode.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use crate::processing_mode::ProcessingMode;
    /// use crate::virtual_directory::VirtualDirectory;
    ///
    /// // Create a virtual directory and add a file path to it
    /// let mut virtual_dir = VirtualDirectory::default();
    /// virtual_dir.add_file(PathBuf::from("dest/sample.txt"));
    ///
    /// // Initialize a ProcessingMode with the virtual directory
    /// let processing_mode = ProcessingMode::DryRun(virtual_dir);
    ///
    /// // Check if the file exists in the virtual directory
    /// let file_path = Path::new("sample.txt");
    /// let dest_path = PathBuf::from("dest");
    /// assert!(processing_mode.contains_file(file_path, &dest_path));
    /// ```
    ///
    /// Note: Replace `your_crate_name` with the actual name of your crate.
    #[allow(dead_code)]
    pub fn contains_file(&self, file_path: &Path, dest_path: &PathBuf) -> bool {
        match self {
            ProcessingMode::DryRun(virtual_dir) => {
                // Convert `file_path` and `dest_path` to a Vec<String> representation.
                let mut path_components = Vec::new();

                // Add the destination path components to the vector.
                if let Some(dest_str) = dest_path.to_str() {
                    path_components.extend(dest_str.split('/').map(String::from));
                }

                // Add the file name to the vector.
                if let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) {
                    path_components.push(file_name.to_string());
                }

                // Use the adapted vector to check if the file is contained within the virtual directory.
                virtual_dir.contains_file(&path_components)
            },
            ProcessingMode::Live => false, // Live processing doesn't inspect a virtual structure.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::processing_mode::ProcessingMode;
    use crate::virtual_directory::VirtualDirectory;
    use std::path::PathBuf;

    #[test]
    fn test_processing_mode_contains_file() {
        let mut virtual_dir = VirtualDirectory::default();
        let path_components = vec!["dest".to_string(), "sample.txt".to_string()];
        virtual_dir.add_path(&path_components);

        let processing_mode = ProcessingMode::DryRun(virtual_dir);

        // Assuming you have a method to convert a Vec<String> back to a PathBuf for this check
        let file_path = PathBuf::from("sample.txt");
        let dest_path = PathBuf::from("dest");

        assert!(processing_mode.contains_file(&file_path, &dest_path));
    }
}
