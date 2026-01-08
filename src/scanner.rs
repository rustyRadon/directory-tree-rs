
use std::{fs, io, path::Path};

use crate::args::Args;

pub fn scan_directory(root: &Path, args: &Args) -> io::Result<()> {
    println!("{}", root.display());
    walk_directory(root, "", args, 0)?;
    Ok(())
}

fn walk_directory(
    path: &Path,
    prefix: &str,
    args: &Args,
    current_depth: usize,
) -> io::Result<()> {
    if current_depth >= args.depth {
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().into_owned();
            
            
            if !args.all && name.starts_with('.') {
                return None;
            }
            
            Some((entry, name))
        })
        .collect();

    entries.sort_by(|(a, _), (b, _)| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for (i, (entry, name)) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        
        let entry_path = entry.path();
        let is_directory = entry_path.is_dir();
        
        
        match (is_directory, args.only_files) {
            (true, true) => (),
            _ => {
                println!("{}{}{}", prefix, connector, name);
                if is_directory {
                    let new_prefix = if is_last {
                        format!("{}    ", prefix)
                    } else {
                        format!("{}│   ", prefix)
                    };
                    walk_directory(&entry_path, &new_prefix, args, current_depth + 1)?;
                }
            }
        }
    }
    
    Ok(())
}