//! ForenX Secure Eraser - Step 4: File Target Resolution & Filesystem Probing.
//!
//! STRICT READ-ONLY GUARANTEE:
//! This crate performs resolution, directory crawling, filesystem feature probing,
//! and safety classification. It does NOT delete, overwrite, truncate, write,
//! lock volumes, dismount volumes, or invoke destructive IOCTLs.
//!
//! Epistemic certainty rule:
//! Values are marked `ProbedValue::Verified(T)` only when authoritatively confirmed
//! by OS queries. Uncertain, unqueried, or failed probes MUST be preserved as
//! `ProbedValue::Unavailable { reason }` and never guessed.

pub mod errors;
pub mod filesystem_probe;
pub mod model;
pub mod platform;
pub mod safety;
pub mod target_resolver;
pub mod traversal;

pub use errors::{EraserError, EraserResult};
pub use filesystem_probe::FilesystemProber;
pub use model::*;
pub use platform::mock::{MockEntryConfig, MockPlatformProvider};
pub use platform::PlatformProvider;
pub use safety::{is_descendant_or_equal, SafetyEngine};
pub use target_resolver::TargetResolver;
pub use traversal::DirectoryCrawler;
