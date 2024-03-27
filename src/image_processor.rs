use crate::metadata::extract_date_from_image;
use crate::organizer::organize_file;
use crate::processing_mode::ProcessingMode;
use crate::traits::processor::Processor;
use chrono::prelude::*;
use std::{fs, io};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct ImageProcessor;

impl Processor for ImageProcessor {
    fn process(&self, path: &PathBuf, destination: &PathBuf, mode: &mut ProcessingMode) {
        let date_based_dir = self.get_destination_subfolder(path);
        let full_destination_dir = destination.join(&date_based_dir);
        if let Err(e) = move_image(path, &full_destination_dir, mode) {
            println!("Error moving image: {}", e);
        }
    }

    fn get_destination_subfolder(&self, path: &PathBuf) -> PathBuf {
        let date_based_subfolder = if let Some(date_str) = extract_date_from_image(path) {
            format_date_to_path(&date_str)
        } else {
            match fs::metadata(path).and_then(|metadata| metadata.modified()) {
                Ok(modified) => {
                    let datetime = system_time_to_date_time(modified);
                    format!("{}/{}", datetime.year(), format!("{:02} - {}", datetime.month(), datetime.format("%B")))
                },
                Err(_) => String::from("Unknown"),
            }
        };
        // Prepend "Photos" directory to the date-based subfolder
        PathBuf::from("Images").join(date_based_subfolder)
    }

}

fn move_image(path: &PathBuf, destination_dir: &PathBuf, mode: &mut ProcessingMode) -> Result<(), io::Error> {
    let destination_path = destination_dir.join(path.file_name().unwrap());

    match mode {
        ProcessingMode::DryRun(virtual_dir) => {
            let path_parts: Vec<String> = destination_path.iter().map(|s| s.to_string_lossy().to_string()).collect();
            virtual_dir.add_path(&path_parts);
            Ok(())
        },
        ProcessingMode::Live => {
            fs::create_dir_all(destination_dir)?;
            organize_file(path, &destination_path, mode)
        }
    }
}

fn format_date_to_path(date_str: &str) -> String {
    // Assuming date_str is in "YYYY:MM:DD HH:MM:SS" format
    let parts: Vec<&str> = date_str
        .split_whitespace()
        .next()
        .unwrap()
        .split(':')
        .collect();
    let year = parts.get(0).unwrap_or(&"UnknownYear");
    let month_num = parts.get(1).unwrap_or(&"00");
    let month = match &**month_num {
        "01" => "01 - January",
        "02" => "02 - February",
        "03" => "03 - March",
        "04" => "04 - April",
        "05" => "05 - May",
        "06" => "06 - June",
        "07" => "07 - July",
        "08" => "08 - August",
        "09" => "09 - September",
        "10" => "10 - October",
        "11" => "11 - November",
        "12" => "12 - December",
        _ => "UnknownMonth",
    };
    format!("{}/{}", year, month)
}

// Utility function to convert SystemTime to DateTime<Local>
fn system_time_to_date_time(local_time: SystemTime) -> DateTime<Local> {
    let datetime: DateTime<Utc> = local_time.into();
    datetime.with_timezone(&Local)
}

#[cfg(test)]
mod image_processor_tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;
    use chrono::{Local, Datelike};

    #[test]
    fn test_image_processor_live() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let destination_dir = temp_dir.path().join("destination");
        fs::create_dir_all(&source_dir).unwrap();
        let image_file_path = source_dir.join("photo1.png");
        let mut file = File::create(&image_file_path).unwrap();
        writeln!(file, "Dummy image content").unwrap();

        // Simulate an ImageProcessor instance and its processing
        let processor = ImageProcessor {};
        let mut mode = ProcessingMode::Live;

        processor.process(&image_file_path, &destination_dir, &mut mode);

        // Dynamically determine the current year and month for the expected path
        let now = Local::now();
        let expected_date_dir = format!("{}/{} - {}", now.year(), format!("{:02}", now.month()), now.format("%B"));
        let expected_destination = destination_dir.join("Images").join(expected_date_dir).join("photo1.png");

        assert!(expected_destination.exists(), "Image was not moved to the correct destination in Live mode.");
    }
}
