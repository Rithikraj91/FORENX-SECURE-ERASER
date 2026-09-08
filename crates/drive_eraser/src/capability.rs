use crate::model::{DriveCapabilities, TriState};

/// Construct a conservative capability descriptor.
///
/// In Step 3, NO destructive probing is performed.
/// Every command state defaults strictly to `TriState::Unknown` unless verified by an external
/// read-only identify payload parser. Bus or media types are NEVER used to extrapolate capability.
pub fn default_readonly_capabilities() -> DriveCapabilities {
    DriveCapabilities {
        ata_secure_erase: TriState::Unknown,
        ata_enhanced_secure_erase: TriState::Unknown,
        ata_sanitize_block: TriState::Unknown,
        ata_sanitize_crypto: TriState::Unknown,
        ata_sanitize_overwrite: TriState::Unknown,
        nvme_format: TriState::Unknown,
        nvme_format_crypto: TriState::Unknown,
        nvme_sanitize_block: TriState::Unknown,
        nvme_sanitize_crypto: TriState::Unknown,
        nvme_sanitize_overwrite: TriState::Unknown,
        scsi_sanitize: TriState::Unknown,
    }
}

/// Evaluates safe, verified identify flags without issuing destructive commands.
/// This helper confirms that without explicit, non-destructive identify data,
/// no capability is promoted to Supported.
pub fn evaluate_safe_capabilities(
    has_ata_identify_evidence: Option<bool>,
    has_nvme_identify_evidence: Option<bool>,
) -> DriveCapabilities {
    let mut caps = default_readonly_capabilities();

    if has_ata_identify_evidence == Some(false) {
        caps.ata_secure_erase = TriState::Unsupported;
        caps.ata_enhanced_secure_erase = TriState::Unsupported;
        caps.ata_sanitize_block = TriState::Unsupported;
        caps.ata_sanitize_crypto = TriState::Unsupported;
        caps.ata_sanitize_overwrite = TriState::Unsupported;
    }

    if has_nvme_identify_evidence == Some(false) {
        caps.nvme_format = TriState::Unsupported;
        caps.nvme_format_crypto = TriState::Unsupported;
        caps.nvme_sanitize_block = TriState::Unsupported;
        caps.nvme_sanitize_crypto = TriState::Unsupported;
        caps.nvme_sanitize_overwrite = TriState::Unsupported;
    }

    caps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_remain_unknown_by_default() {
        let caps = default_readonly_capabilities();
        assert_eq!(caps.ata_secure_erase, TriState::Unknown);
        assert_eq!(caps.nvme_sanitize_crypto, TriState::Unknown);
        assert_eq!(caps.scsi_sanitize, TriState::Unknown);
    }

    #[test]
    fn test_no_false_positive_from_bus_type() {
        // Confirm that initializing caps does not mark NVMe or ATA as supported
        let caps = evaluate_safe_capabilities(None, None);
        assert_eq!(caps.nvme_format, TriState::Unknown);
        assert_eq!(caps.ata_secure_erase, TriState::Unknown);
    }

    #[test]
    fn test_negative_evidence_marks_unsupported() {
        let caps = evaluate_safe_capabilities(Some(false), Some(false));
        assert_eq!(caps.ata_secure_erase, TriState::Unsupported);
        assert_eq!(caps.nvme_format, TriState::Unsupported);
        assert_eq!(caps.scsi_sanitize, TriState::Unknown);
    }
}