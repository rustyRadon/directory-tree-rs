use std::path::Path;

pub fn validate_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err("path does not exist".to_string());
    }
    if !path.is_dir() {
        return Err("directory does not exist".to_string());
    }
    
    Ok(())
}