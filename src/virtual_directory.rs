use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct VirtualDirectory {
    files: Vec<String>,
    directories: HashMap<String, VirtualDirectory>,
}

impl VirtualDirectory {
    pub fn add_path(&mut self, parts: &[String]) {
        if parts.is_empty() {
            return;
        }

        let (first, rest) = parts.split_first().unwrap();
        if rest.is_empty() {
            self.files.push(first.clone());
        } else {
            self.directories
                .entry(first.clone())
                .or_insert_with(VirtualDirectory::default)
                .add_path(rest);
        }
    }

    pub fn print(&self, prefix: &str) {
        let mut entries = self.directories.iter().collect::<Vec<_>>();
        entries.sort_by_key(|e| e.0);

        let total_entries = entries.len() + self.files.len();
        let mut current_index = 0; // Keep track of the current index across both directories and files

        for (dir, sub_dir) in entries.iter() {
            let is_current_last = current_index == total_entries - 1;
            let connector = if is_current_last {
                "└── "
            } else {
                "├── "
            };
            println!("{}{}{}", prefix, connector, dir);
            let new_prefix = if is_current_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            sub_dir.print(&new_prefix);
            current_index += 1; // Increment the index after processing a directory
        }

        for (i, file) in self.files.iter().enumerate() {
            let is_current_last = current_index + i == total_entries - 1;
            let connector = if is_current_last {
                "└── "
            } else {
                "├── "
            };
            println!("{}{}{}", prefix, connector, file);
        }
    }
    // Wrapper function to start the printing process without external parameters
    pub fn print_tree(&self) {
        self.print("");
    }

    #[allow(dead_code)]
    pub fn contains_file(&self, path: &[String]) -> bool {
        if path.is_empty() {
            return false;
        }

        let (first, rest) = path.split_first().unwrap();
        if rest.is_empty() {
            // We're at the last component, which should be a file.
            self.files.contains(first)
        } else {
            // We're looking at a directory; dive deeper.
            if let Some(sub_dir) = self.directories.get(first) {
                sub_dir.contains_file(rest)
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
use crate::virtual_directory::VirtualDirectory;

    #[test]
    fn test_add_path_to_virtual_directory() {
        let mut virtual_dir = VirtualDirectory::default();
        let path_components = vec!["destination".to_string(), "Word_Documents".to_string(), "document.docx".to_string()];
        virtual_dir.add_path(&path_components);

        // Debug print to verify structure after addition
        println!("{:#?}", virtual_dir);

        // Assertions to ensure the path was added correctly
        assert!(!virtual_dir.directories.is_empty(), "Directories should not be empty.");
        assert!(virtual_dir.contains_file(&path_components), "Path should exist in VirtualDirectory.");
    }

    #[test]
    fn direct_virtual_directory_test() {
        let mut virtual_dir = VirtualDirectory::default();
        let path_components = vec!["destination".to_string(), "Word_Documents".to_string(), "document.docx".to_string()];
        virtual_dir.add_path(&path_components);

        assert!(virtual_dir.contains_file(&path_components),
            "VirtualDirectory does not contain the expected path after direct addition.");
    }

}
