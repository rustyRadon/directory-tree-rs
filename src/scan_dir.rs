use std::fs;
use std::path::Path;

pub fn print_tree(
    path: &Path,
    prefix: &str,
    files_only: bool,
    current_depth: usize,
    max_depth: usize,
) {
    // Stop if current depth reached max depth
    if current_depth >= max_depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .collect();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let symbol = if is_last { "└── " } else { "├── " };

        let entry_path = entry.path();
        
        // Store the file_name in a variable to extend its lifetime
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if entry_path.is_dir() {
            if !files_only {
                println!("{}{}{}", prefix, symbol, name);
            }
            
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            // Recurse deeper
            print_tree(
                &entry_path,
                &new_prefix,
                files_only,
                current_depth + 1,
                max_depth,
            );
        } else {
            println!("{}{}{}", prefix, symbol, name);
        }
    }
}