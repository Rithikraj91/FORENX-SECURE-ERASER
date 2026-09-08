//! ForenX Secure Drive Eraser.
//!
//! Step 3 provides read-only drive discovery, identification,
//! classification, capability assessment, and safety metadata.
//!
//! No destructive drive operations are implemented in this module yet.

pub mod capability;
pub mod classify;
pub mod discovery;
pub mod error;
pub mod identity;
pub mod model;

pub use error::{DriveEraserError, Result};