
use std::{io, path::Path};

pub fn validate_path(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist",
        ));
    }
    
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }
    
    Ok(())
}