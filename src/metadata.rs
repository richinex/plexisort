use exif::{In, Reader, Tag};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn extract_date_from_image(path: &Path) -> Option<String> {
    // Open the file at the given path
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    // Create a BufReader for the file
    let mut buf_reader = BufReader::new(file);

    // Create an EXIF reader and attempt to read the EXIF data from the BufReader
    let exif_reader = Reader::new().read_from_container(&mut buf_reader);

    match exif_reader {
        Ok(exif) => {
            // Attempt to get the 'DateTimeOriginal' field from the EXIF data
            let field = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY);

            // If the field exists, return its display value as a String
            field.map(|field| field.display_value().to_string())
        }
        Err(_) => None, // If there was an error reading the EXIF data, return None
    }
}
