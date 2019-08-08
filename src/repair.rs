//! Definitions related to repair functionality.
use std::time::Duration;

/// A value that eventually goes into Synchronizer::repair_idleness_threshold.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepairIdleness {
    /// Repair should wait for the given duration of idleness.
    Threshold(Duration),
    /// Repair is disabled.
    Disabled,
}

/// The maximum number of threads to execute repairing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RepairConcurrencyLimit(pub u64);

/// The maximum number of threads to execute full_sync.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FullSyncConcurrencyLimit(pub u64);

/// Configuration of frugalos_segment's repair functionality.
/// If a field is None, that field will remain unchanged.
/// If a field is Some(val), that field will change to val.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairConfig {
    /// SegmentService::repair_concurrency_limit
    pub repair_concurrency_limit: Option<RepairConcurrencyLimit>,
    /// Synchronizer::repair_idleness_threshold
    pub repair_idleness_threshold: Option<RepairIdleness>,
    /// SegmentService::full_sync_concurrency_limit
    pub full_sync_concurrency_limit: Option<FullSyncConcurrencyLimit>,
}
