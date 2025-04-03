use std::path::PathBuf;

use glob::glob;

pub fn find(path: &str) -> Vec<PathBuf> {
    let mut kt_files: Vec<PathBuf> = Vec::new();
    
    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => kt_files.push(path),
            Err(e) => println!("{:?}", e),
        }
    }
    
    kt_files
}