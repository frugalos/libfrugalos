//! セグメント関連のエンティティ定義。

/// セグメント統計情報。
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SegmentStatistics {
    /// 実際に計測されたストレージ使用量。
    pub storage_usage_bytes_real: u64,
    /// 推定されるストレージ使用量。
    pub storage_usage_bytes_approximation: u64,
}
