//! ForenX Secure File & Metadata Eraser — crate scaffold.
//!
//! STATUS: Placeholder only. No target resolution, filesystem probing,
//! content-overwrite, metadata sanitization, or slack-space handling
//! logic has been implemented yet. No destructive functionality exists
//! in this crate.
//!
//! This file exists solely so the workspace compiles as a valid crate
//! boundary for future implementation work.

/// Placeholder marker confirming this crate compiles as part of the
/// ForenX Secure Eraser workspace. Will be removed once real module
/// code is added.
pub fn scaffold_marker() -> &'static str {
    "file_eraser scaffold ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_compiles() {
        assert_eq!(scaffold_marker(), "file_eraser scaffold ok");
    }
}
