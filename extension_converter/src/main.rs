use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage 1 (Directory): {} <directory> <old_extension> <new_extension>", args[0]);
        eprintln!("Usage 2 (Single File): {} <file> <new_extension>", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} ./images jpeg jpg", args[0]);
        eprintln!("  {} my_document.txt md", args[0]);
        process::exit(1);
    }

    let target = &args[1];
    let target_path = Path::new(target);

    if !target_path.exists() {
        eprintln!("Error: '{}' does not exist.", target);
        process::exit(1);
    }

    if target_path.is_file() {
        // Mode 1: Single file conversion
        let new_ext = &args[2];
        let new_ext = new_ext.trim_start_matches('.');

        let mut new_path = target_path.to_path_buf();
        new_path.set_extension(new_ext);

        match fs::rename(&target_path, &new_path) {
            Ok(_) => {
                println!("Renamed: {:?} -> {:?}", target_path.file_name().unwrap(), new_path.file_name().unwrap());
            }
            Err(e) => eprintln!("Failed to rename {:?}: {}", target_path, e),
        }
    } else if target_path.is_dir() {
        // Mode 2: Directory batch conversion
        if args.len() < 4 {
            eprintln!("Error: When passing a directory, you must provide both the old and new extensions.");
            eprintln!("Example: {} ./my_folder txt md", args[0]);
            process::exit(1);
        }

        let old_ext = args[2].trim_start_matches('.');
        let new_ext = args[3].trim_start_matches('.');
        let mut count = 0;

        match fs::read_dir(target_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if ext.to_string_lossy().eq_ignore_ascii_case(old_ext) {
                                    let mut new_path = path.clone();
                                    new_path.set_extension(new_ext);
                                    
                                    match fs::rename(&path, &new_path) {
                                        Ok(_) => {
                                            println!("Renamed: {:?} -> {:?}", path.file_name().unwrap(), new_path.file_name().unwrap());
                                            count += 1;
                                        }
                                        Err(e) => eprintln!("Failed to rename {:?}: {}", path, e),
                                    }
                                }
                            }
                        }
                    }
                }
                println!("Successfully changed the extension of {} files.", count);
            }
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
                process::exit(1);
            }
        }
    }
}
