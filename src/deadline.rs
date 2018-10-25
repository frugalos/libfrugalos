//! RPC処理のデッドライン。
use std::time::Duration;

/// RPC処理のデッドラインを表現するための構造体。
#[derive(Debug, Clone, Copy)]
pub struct Deadline(Duration);
impl Deadline {
    /// 新しい`Deadline`インスタンスを生成する。
    pub fn new(duration: Duration) -> Self {
        Deadline(duration)
    }

    /// デッドラインを`Duration`に変換して返す。
    pub fn as_duration(&self) -> Duration {
        self.0
    }
}
