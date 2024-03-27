use std::{path::PathBuf, sync::Mutex};

use crate::{compressed_file_processor::CompressedFileProcessor, document_processor::DocumentProcessor, image_processor::ImageProcessor, generic_processor::GenericProcessor, video_processor::VideoProcessor};

use self::processor::Processor;
use mime_guess::from_path;

pub mod processor;


pub trait ProcessorFactory {
    fn create_processor(&self, path: &PathBuf) -> Box<dyn Processor>;
}


pub struct DefaultProcessorFactory;

impl ProcessorFactory for DefaultProcessorFactory {
    fn create_processor(&self, path: &PathBuf) -> Box<dyn Processor> {
        let mime_type = from_path(path).first_or_octet_stream();
        let file_extension = path.extension().unwrap_or_default().to_str().unwrap_or("").to_lowercase();

        match mime_type.type_() {
            mime::IMAGE => Box::new(ImageProcessor),
            mime::VIDEO => Box::new(VideoProcessor),
            mime::TEXT => Box::new(DocumentProcessor),
            mime::APPLICATION => match file_extension.as_str() {
                "pdf" | "doc" | "docx" | "ppt" | "pptx" | "xlsx" | "xls" | "json" | "yml" => Box::new(DocumentProcessor),
                "zip" | "tar" | "rar" | "7z" => Box::new(CompressedFileProcessor),
                _ => Box::new(GenericProcessor),
            },
            _ => Box::new(GenericProcessor),
        }
    }
}


pub struct TestProcessorFactory {
    // Used in tests to check which processor was created last
    pub last_processor_type: Mutex<Option<String>>,
}

impl ProcessorFactory for TestProcessorFactory {
    fn create_processor(&self, path: &PathBuf) -> Box<dyn Processor> {
        let file_extension = path.extension().unwrap_or_default().to_str().unwrap_or("").to_lowercase();
        let processor = match file_extension.as_str() {
            "jpg" | "png" => {
                self.last_processor_type.lock().unwrap().replace("ImageProcessor".to_string());
                Box::new(ImageProcessor) as Box<dyn Processor>
            },
            "docx" | "txt" => {
                self.last_processor_type.lock().unwrap().replace("DocumentProcessor".to_string());
                Box::new(DocumentProcessor) as Box<dyn Processor>
            },
            // Add other cases as necessary
            _ => {
                self.last_processor_type.lock().unwrap().replace("UnknownProcessor".to_string());
                Box::new(GenericProcessor) as Box<dyn Processor>
            },
        };
        processor
    }
}


