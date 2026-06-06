//! Compile-time app config, derived from tauri.conf.json by build.rs.

/// The document file extension (e.g. "appdoc"), without the dot.
pub const DOC_EXT: &str = env!("APP_DOC_EXT");

/// Build date (YYYY-MM-DD), for the About dialog.
pub const BUILD_DATE: &str = env!("BUILD_DATE");

/// Find the first argument that looks like one of our documents
/// (used to handle "open with" on Windows/Linux).
pub fn doc_path_from_args<I: IntoIterator<Item = String>>(args: I) -> Option<String> {
    let suffix = format!(".{DOC_EXT}");
    args.into_iter()
        .find(|a| a.to_lowercase().ends_with(&suffix))
}
