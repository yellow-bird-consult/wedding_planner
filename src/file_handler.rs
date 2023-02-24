//! the file handler is for managing the interface for basic file operations.
use std::fs;
use std::path::Path;


/// The core file handle trait for actions on files.
#[mockall::automock]
pub trait CoreFileHandle {

    fn copy(&self, from: &Path, to: &Path) -> Result<u64, std::io::Error>;

    fn remove(&self, path: &Path) -> Result<(), std::io::Error>;

    fn create_directory_if_not_exists(&self, path: &Path) -> Result<(), std::io::Error>;

}


pub struct FileHandle {}


impl CoreFileHandle for FileHandle {

    /// Copies a file from one location to another.
    /// 
    /// # Arguments
    /// * `from` - The path to the file to copy
    /// * `to` - The path to copy the file to
    /// 
    /// # Returns
    /// * `Result<u64, std::io::Error>` - The number of bytes copied or an error
    fn copy(&self, from: &Path, to: &Path) -> Result<u64, std::io::Error> {
        fs::copy(from, to)
    }

    /// Removes a file from the file system.
    /// 
    /// # Arguments
    /// * `path` - The path to the file to remove
    /// 
    /// # Returns
    /// * `Result<(), std::io::Error>` - An error if the file could not be removed
    fn remove(&self, path: &Path) -> Result<(), std::io::Error> {
        fs::remove_file(path)
    }

    /// Creates a directory if it does not already exist.
    /// 
    /// # Arguments
    /// * `path` - The path to the directory to create
    /// 
    /// # Returns
    /// * `Result<(), std::io::Error>` - An error if the directory could not be created
    fn create_directory_if_not_exists(&self, path: &Path) -> Result<(), std::io::Error> {
        if !fs::metadata(path).is_ok() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

}

