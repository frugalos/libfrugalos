//! Compare-And-Swap用の構成要素。
use entity::object::ObjectVersion;
use {ErrorKind, Result};

/// 操作対象オブジェクトに期待するバージョンを表現するためのデータ構造.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expect {
    /// 任意のバージョンに対して適用可能.
    Any,

    /// オブジェクトが既に存在しない場合にのみ適用可能.
    None,

    /// 指定バージョンのオブジェクトに対してのみ適用可能.
    IfMatch(Vec<ObjectVersion>),

    /// オブジェクトのバージョンが指定のもの以外の場合にのみ適用可能.
    IfNoneMatch(Vec<ObjectVersion>),
}
impl Expect {
    /// 引数で指定されたバージョンが、期待するものかどうかを検証する。
    pub fn validate(&self, version: Option<ObjectVersion>) -> Result<()> {
        match *self {
            Expect::Any => {}
            Expect::None => track_assert_eq!(version, None, ErrorKind::Unexpected(version)),
            Expect::IfMatch(ref versions) => track_assert!(
                versions.iter().any(|&v| Some(v) == version),
                ErrorKind::Unexpected(version)
            ),
            Expect::IfNoneMatch(ref versions) => track_assert!(
                versions.iter().all(|&v| Some(v) != version),
                ErrorKind::Unexpected(version)
            ),
        }
        Ok(())
    }
}
impl Default for Expect {
    fn default() -> Self {
        Expect::Any
    }
}
