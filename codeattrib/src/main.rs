use clap::Parser; // For command-line parsing
use std::fs::{read_dir, OpenOptions}; // For file handling
use std::io::{Read, Write}; // For reading and writing file contents
use std::path::PathBuf; // For handling paths

use codeattrib::{detect_language, generate_comment, Language, SourceInfo}; // Import from lib.rs

// Struct to hold all supported command line arguments
#[derive(Parser)]
struct Args {
    #[clap()]
    directory: PathBuf, // Automatically parses directory as a PathBuf.

    #[clap(short = 'l', long = "languages", value_enum)]
    languages: Vec<Language>, // The list of languages to filter files by.

                              // TODO: add github url
}

fn main() -> std::io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    for entry in read_dir(args.directory)? {
        // Extract path from directory entry.
        let path = entry?.path();

        // Skip non-file paths.
        // TODO: add optional recursion flag
        if !path.is_file() {
            continue;
        }

        // Detect programming language based on file extension.
        let language = match detect_language(&path) {
            Some(lang) => lang,
            None => {
                println!("Language not detected for file: {:?}", path);
                continue;
            }
        };

        // Check if specific languages were specified and skip files not in the list.
        if !args.languages.contains(&Language::All) && !args.languages.contains(&language) {
            println!(
                "File skipped as it's not in the selected languages: {:?}",
                path
            );
            continue;
        }

        // Mock project source information.
        // TODO: extract from given github url
        let source_info = SourceInfo {
            project_name: "Example Project".to_string(),
            project_url: "https://example.com".to_string(),
            authors: vec!["Author1".to_string(), "Author2".to_string()],
            license: "MIT".to_string(),
        };

        // Generate the appropriate comment based on the detected language and project info.
        let comment = generate_comment(&language, &source_info);

        // Print the generated comment for debug
        println!("Generated comment for {:?}: {}", path, comment);

        // Read existing content of the file.
        let mut content = String::new();
        {
            let mut file = match OpenOptions::new().read(true).open(&path) {
                Ok(file) => file,
                Err(e) => {
                    println!("Failed to open file for reading: {:?}, error: {}", path, e);
                    continue;
                }
            };
            if let Err(e) = file.read_to_string(&mut content) {
                println!("Failed to read file content: {:?}, error: {}", path, e);
                continue;
            }
        }

        // Prepend the generated comment to the file content.
        let modified_content = format!("{}{}", comment, content);

        // Open the file in writing mode
        {
            let mut file = match OpenOptions::new().write(true).truncate(true).open(&path) {
                Ok(file) => file,
                Err(e) => {
                    println!("Failed to open file for writing: {:?}, error: {}", path, e);
                    continue;
                }
            };

            // Write the modified content back to the file.
            if let Err(e) = file.write_all(modified_content.as_bytes()) {
                println!("Failed to write to file {:?}: {}", path, e);
            } else {
                // Ensure the written data is flushed to disk.
                if let Err(e) = file.flush() {
                    println!("Failed to flush content to file {:?}: {}", path, e);
                } else {
                    println!("File {:?} updated successfully.", path);
                }
            }
        }
    }

    Ok(()) // Return Ok if everything succeeded.
}

//********//
// TESTS  //
//********//
#[cfg(test)]
mod tests {
    use super::*;

    // Test the generate_comment function for Python files.
    #[test]
    fn test_generate_comment_python() {
        let source_info = SourceInfo {
            project_name: "Test Project".to_string(),
            project_url: "https://testproject.com".to_string(),
            authors: vec!["Test Author".to_string()],
            license: "Apache-2.0".to_string(),
        };

        let comment = generate_comment(&Language::Python, &source_info);
        let expected_comment = "# Copied from Test Project (https://testproject.com)\n# Original authors: Test Author\n# License: Apache-2.0\n\n";

        assert_eq!(comment, expected_comment); // Assert that the generated comment matches the expected one.
    }

    // Test the generate_comment function for Rust files.
    #[test]
    fn test_generate_comment_rust() {
        let source_info = SourceInfo {
            project_name: "Rust Project".to_string(),
            project_url: "https://rustproject.com".to_string(),
            authors: vec!["Rustacean".to_string()],
            license: "MIT".to_string(),
        };

        let comment = generate_comment(&Language::Rust, &source_info);
        let expected_comment = "// Copied from Rust Project (https://rustproject.com)\n// Original authors: Rustacean\n// License: MIT\n\n";

        assert_eq!(comment, expected_comment);
    }

    // Test the generate_comment function for Java files.
    #[test]
    fn test_generate_comment_java() {
        let source_info = SourceInfo {
            project_name: "Java Project".to_string(),
            project_url: "https://javaproject.com".to_string(),
            authors: vec!["Java Dev".to_string()],
            license: "GPL-3.0".to_string(),
        };

        let comment = generate_comment(&Language::Java, &source_info);
        let expected_comment = "/* Copied from Java Project (https://javaproject.com)\n * Original authors: Java Dev\n * License: GPL-3.0 */\n\n";

        assert_eq!(comment, expected_comment);
    }
}
