//! オブジェクト関連のエンティティ定義。
use std::str::FromStr;
use Error;

// FIXME: 構造体にする
/// オブジェクトのID。
pub type ObjectId = String;

/// メタデータオブジェクトのバージョン.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ObjectVersion(pub u64);

impl FromStr for ObjectVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track!(s.parse::<u64>().map(ObjectVersion).map_err(Error::from))
    }
}

/// メタデータオブジェクトの接頭辞
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectPrefix(pub String);

/// メタデータオブジェクトの要約.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ObjectSummary {
    pub id: String,
    pub version: ObjectVersion,
}

/// オブジェクトのメタデータ.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// バージョン番号.
    pub version: ObjectVersion,

    /// ユーザ定義の任意のバイト列.
    pub data: Vec<u8>,
}

/// 接頭辞指定でのオブジェクト削除時の削除結果要約
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteObjectsByPrefixSummary {
    /// 削除されたオブジェクトの総数
    /// JSON で u64 を扱えるように string に serialize する。
    #[serde(with = "prefix_summary_total")]
    pub total: u64,
}

mod prefix_summary_total {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::fmt::Display;
    use std::str::FromStr;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(x: &u64, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&x.to_string())
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}
