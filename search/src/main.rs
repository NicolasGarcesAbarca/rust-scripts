use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: <program> <directory> <extension> <target_string> <exclude_folder>");
        return Ok(());
    }

    let dir_path = &args[1];
    let file_extension = &args[2];
    let target_string = &args[3];
    let exclude_folder = &args[4];

    // Ensure the provided path is valid
    let dir = Path::new(dir_path);
    if !dir.is_dir() {
        eprintln!("Error: {} is not a valid directory", dir_path);
        return Ok(());
    }

    // Recursively search files and look for the target string
    find_in_files(dir, file_extension, target_string, exclude_folder)?;

    Ok(())
}

/// Recursively search for files with the given extension and check for the target string
fn find_in_files(
    dir: &Path,
    extension: &str,
    target: &str,
    exclude_folder: &str,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectories
            // match file_name Option<&OsStr>
            if let Some(folder_name) = path.file_name(){
                if folder_name == exclude_folder{
                    println!("Skipping folder:{}",path.display());
                    continue;
                }
            }
            find_in_files(&path, extension, target,exclude_folder)?;
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                search_file(&path, target)?;
            }
        }
    }
    Ok(())
}

/// Search for the target string inside a file
fn search_file(file_path: &Path, target: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(target) {
            println!(
                "Found '{}' in file {} at line {}",
                target,
                file_path.display(),
                line_num + 1
            );
        }
    }

    Ok(())
}
