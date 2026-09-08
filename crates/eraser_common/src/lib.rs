//! ForenX Secure Eraser — shared/common crate scaffold.
//!
//! STATUS: Placeholder only. No audit logging, SHA-256 hashing helpers,
//! SQLite/RocksDB storage adapters, or shared IPC DTOs have been
//! implemented yet.
//!
//! This file exists solely so the workspace compiles as a valid crate
//! boundary for future implementation work.

/// Placeholder marker confirming this crate compiles as part of the
/// ForenX Secure Eraser workspace. Will be removed once real module
/// code is added.
pub fn scaffold_marker() -> &'static str {
    "eraser_common scaffold ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_compiles() {
        assert_eq!(scaffold_marker(), "eraser_common scaffold ok");
    }
}
