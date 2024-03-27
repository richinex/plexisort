use std::path::PathBuf;
use crate::processing_mode::ProcessingMode;

pub trait Processor {
    fn process(&self, path: &PathBuf, destination: &PathBuf, mode: &mut ProcessingMode);
    fn get_destination_subfolder(&self, path: &PathBuf) -> PathBuf; // New method
}

