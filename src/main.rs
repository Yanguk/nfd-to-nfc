use clap::{App, Arg};
use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;
use walkdir::WalkDir;

fn main() {
    let matches = App::new("NFD to NFC Filename Converter")
        .version("1.0")
        .author("Your Name")
        .about("Convert filenames from NFD to NFC Unicode normalization form")
        .arg(
            Arg::with_name("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("Directory to process (default: current directory)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recursive")
                .short('r')
                .long("recursive")
                .help("Process directories recursively"),
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .help("Show what would be done without actually renaming files"),
        )
        .get_matches();

    let current_dir = env::current_dir().expect("Failed to get current directory");

    let directory = match matches.value_of("directory") {
        Some(dir) => PathBuf::from(dir),
        None => current_dir,
    };

    let recursive = matches.is_present("recursive");
    let dry_run = matches.is_present("dry-run");

    println!(
        "{} {} {}",
        "NFD to NFC Filename Converter".bright_green().bold(),
        "v1.0".bright_blue(),
        if dry_run {
            "[DRY RUN]".bright_yellow()
        } else {
            "".normal()
        }
    );
    println!(
        "Processing directory: {}",
        directory.display().to_string().cyan()
    );

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
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let parent = path.parent().unwrap();

                    let nfc_file_name = file_name.nfc().collect::<String>();

                    if file_name != nfc_file_name {
                        converted_files += 1;
                        let new_path = parent.join(&nfc_file_name);

                        println!(
                            "{} '{}' {} '{}'",
                            "Renaming:".bright_yellow(),
                            path.display().to_string().red(),
                            "→".bright_white(),
                            new_path.display().to_string().green()
                        );

                        if !dry_run {
                            if let Err(e) = fs::rename(path, &new_path) {
                                eprintln!("Error renaming file {}: {}", path.display(), e);
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

    println!("\n{} {}", "Summary:".bright_cyan().bold(), "-".repeat(30));
    println!(
        "{}: {}",
        "Total files/directories processed".white(),
        total_files.to_string().bright_white()
    );
    println!(
        "{}: {}",
        "Files/directories converted".white(),
        converted_files.to_string().bright_white()
    );

    if dry_run {
        println!(
            "\n{}",
            "This was a dry run. No files were actually renamed.".bright_yellow()
        );
        println!(
            "{}",
            "Run without --dry-run to perform the conversion.".bright_yellow()
        );
    }
}
