//! 多重度に関する設定をまとめたモジュール。

/// 多重度に関する設定。
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MultiplicityConfig {
    pub inner_retry_count: InnerRetryCount,
    pub number_of_ensured_saves: NumberOfEnsuredSaves,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// How many times frugalos_segment retries to save the object.
/// Defaults to 0.
pub struct InnerRetryCount(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Number of objects ensured to be saved, apart from data_fragment.
/// Defaults to 0.
pub struct NumberOfEnsuredSaves(pub usize);
