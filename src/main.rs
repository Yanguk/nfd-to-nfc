use pico_args::Arguments;
use std::env;
use std::fs;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;
use walkdir::WalkDir;

const HELP: &str = "\
NFD to NFC Filename Converter

Usage:
  nfd_to_nfc [options]

Options:
  -d, --directory DIR  Directory to process (default: current directory)
  -r, --recursive      Process directories recursively
  --dry-run            Show what would be done without actually renaming files
  -h, --help           Show this help
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        println!("{}", HELP);
        return Ok(());
    }

    let directory: PathBuf = args
        .opt_value_from_str(["-d", "--directory"])?
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    let recursive = args.contains(["-r", "--recursive"]);
    let dry_run = args.contains("--dry-run");

    println!("NFD to NFC Filename Converter");
    if dry_run {
        println!("[DRY RUN]");
    }
    println!("Processing directory: {}", directory.display());

    let mut total_files = 0;
    let mut converted_files = 0;

    let walker = if recursive {
        WalkDir::new(&directory)
    } else {
        WalkDir::new(&directory).max_depth(1)
    };

    for entry in walker {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() || entry.file_type().is_dir() {
                    total_files += 1;

                    let path = entry.path();

                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            let nfc_file_name = file_name_str.nfc().collect::<String>();

                            if file_name_str != nfc_file_name {
                                converted_files += 1;
                                let parent = path.parent().unwrap();
                                let new_path = parent.join(&nfc_file_name);

                                println!(
                                    "Renaming: '{}' → '{}'",
                                    path.display(),
                                    new_path.display()
                                );

                                if !dry_run {
                                    if let Err(e) = fs::rename(path, &new_path) {
                                        eprintln!("Error renaming file {}: {}", path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error accessing entry: {}", e);
            }
        }
    }

    println!("\nSummary:");
    println!("Total files/directories processed: {}", total_files);
    println!("Files/directories converted: {}", converted_files);

    if dry_run {
        println!("\nThis was a dry run. No files were actually renamed.");
        println!("Run without --dry-run to perform the conversion.");
    }

    Ok(())
}
