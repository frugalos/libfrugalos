//! セグメント関連のエンティティ定義。

/// セグメント統計情報。
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SegmentStatistics {
    /// ストレージ使用量(バイト)。
    pub storage_usage_bytes: u64,
}
