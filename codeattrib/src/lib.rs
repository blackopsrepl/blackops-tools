use std::path::Path;

use clap::ValueEnum;

// Struct to hold source project information.
#[derive(Debug, PartialEq)]
pub struct SourceInfo {
    pub project_name: String, // The name of the project the source is copied from.
    pub project_url: String,  // The URL of the project.
    pub authors: Vec<String>, // A list of original authors of the source.
    pub license: String,      // The license under which the project is released.
}

// Enum to hold supported programming languages.
// ValueEnum derives allows clap to automatically convert command-line arguments into enum values.
#[derive(Clone, ValueEnum, PartialEq, Debug)]
pub enum Language {
    Python,
    Rust,
    Java,
    All,
}
// Detects programming language from file extensions.
pub fn detect_language(path: &Path) -> Option<Language> {
    if let Some(extension) = path.extension() {
        match extension.to_str() {
            Some("py") => Some(Language::Python),
            Some("rs") => Some(Language::Rust),
            Some("java") => Some(Language::Java),
            _ => None,
        }
    } else {
        None
    }
}

// Generates the header block based on the detected programming language and project source information.
pub fn generate_comment(language: &Language, source_info: &SourceInfo) -> String {
    match language {
        Language::Python => format!(
            "# Copied from {project_name} ({project_url})\n# Original authors: {authors}\n# License: {license}\n\n",
            project_name = source_info.project_name,
            project_url = source_info.project_url,
            authors = source_info.authors.join(", "),
            license = source_info.license
        ),
        Language::Rust => format!(
            "// Copied from {project_name} ({project_url})\n// Original authors: {authors}\n// License: {license}\n\n",
            project_name = source_info.project_name,
            project_url = source_info.project_url,
            authors = source_info.authors.join(", "),
            license = source_info.license
        ),
        Language::Java => format!(
            "/* Copied from {project_name} ({project_url})\n * Original authors: {authors}\n * License: {license} */\n\n",
            project_name = source_info.project_name,
            project_url = source_info.project_url,
            authors = source_info.authors.join(", "),
            license = source_info.license
        ),
        _ => String::new(),
    }
}
