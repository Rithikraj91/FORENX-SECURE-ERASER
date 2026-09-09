//! ForenX Secure Eraser - Step 4: File Target Resolution & Filesystem Probing.
//!
//! STRICT READ-ONLY GUARANTEE:
//! This crate performs resolution, directory crawling, filesystem feature probing,
//! safety classification, and read-only capability assessment (Step 5.1).
//! It does NOT delete, overwrite, truncate, write, lock volumes, dismount volumes,
//! or invoke destructive IOCTLs.
//!
//! Epistemic certainty rule:
//! Values are marked `ProbedValue::Verified(T)` or `CapabilityState::Supported` only
//! when authoritatively confirmed by OS queries or verified evidence. Uncertain,
//! unqueried, or failed probes MUST be preserved as `ProbedValue::Unavailable` or
//! `CapabilityState::Unknown` and never guessed.

pub mod capability;
pub mod errors;
pub mod filesystem_probe;
pub mod model;
pub mod platform;
pub mod safety;
pub mod target_resolver;
pub mod traversal;

pub use capability::{
    assess_target_capabilities, CapabilityFinding, CapabilityState, FileCapabilityAssessment,
};
pub use errors::{EraserError, EraserResult};
pub use filesystem_probe::FilesystemProber;
pub use model::*;
pub use platform::mock::{MockEntryConfig, MockPlatformProvider};
pub use platform::PlatformProvider;
pub use safety::{is_descendant_or_equal, SafetyEngine};
pub use target_resolver::TargetResolver;
pub use traversal::DirectoryCrawler;