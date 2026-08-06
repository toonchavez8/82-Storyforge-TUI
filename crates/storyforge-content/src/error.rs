use std::{io, path::PathBuf};

/// Errors that can occur while loading or validating campaign content.
///
/// The goal is to provide enough context for developers and content authors
/// to quickly identify which file caused the problem and why.
#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    /// A file could not be read from disk.
    /// This usually indicates a missing file or a filesystem permission issue.
    #[error("could not read `{path}`: {source}")]
    Read {
        /// File that failed to load.
        path: PathBuf,

        /// Original I/O error returned by the standard library.
        #[source]
        source: io::Error,
    },

    /// The campaign manifest exists but contains invalid TOML.
    #[error("invalid campaign manifest `{path}`: {source}")]
    Manifest {
        /// Manifest file that failed to parse.
        path: PathBuf,

        /// Parsing error reported by the TOML parser.
        #[source]
        source: toml::de::Error,
    },

    /// A scene file contains invalid RON syntax or does not match
    /// the expected schema.
    #[error("invalid scene `{path}`: {source}")]
    Scene {
        /// Scene file that could not be parsed.
        path: PathBuf,

        /// Detailed parsing error, including location information.
        #[source]
        source: Box<ron::error::SpannedError>,
    },

    /// One or more validation rules failed after the campaign
    /// was successfully loaded.
    #[error("campaign validation failed with {0} error(s)")]
    Validation(usize),
}

/// A single validation message produced while checking campaign content.
///
/// Diagnostics are collected instead of immediately failing so authors
/// can fix multiple issues in a single pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Indicates whether this is a warning or an error.
    pub severity: Severity,

    /// Stable identifier that tools can use for filtering or documentation.
    pub code: &'static str,

    /// Human-readable explanation of the issue.
    pub message: String,

    /// File associated with the diagnostic, when applicable.
    pub path: Option<PathBuf>,
}

/// Describes how serious a diagnostic is.
///
/// Warnings highlight potential issues that do not prevent the campaign
/// from running, while errors indicate problems that must be fixed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Something looks suspicious but is not fatal.
    Warning,

    /// A problem that prevents the content from being considered valid.
    Error,
}
