//! デバイス関連のエンティティ定義。
use libc;
use std;
use std::collections::BTreeSet;
use std::path::PathBuf;

use entity::server::ServerId;
use {Error, ErrorKind, Result};

// FIXME: 構造体に置き換える
/// デバイスのID。
pub type DeviceId = String;

/// デバイスの内容の要約。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSummary {
    /// デバイスのID。
    pub id: DeviceId,

    /// デバイスを保持しているサーバ。
    ///
    /// 仮想デバイスの場合には`None`となる。
    #[serde(default)]
    pub server: Option<ServerId>,

    /// デバイスの種類。
    #[serde(rename = "type")]
    pub kind: DeviceKind,
}

/// デバイスの種類。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceKind {
    /// 仮想デバイス。
    Virtual,

    /// メモリデバイス。
    Memory,

    /// ファイルデバイス。
    File,
}

/// デバイス。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Device {
    /// 仮想デバイス。
    Virtual(VirtualDevice),

    /// メモリデバイス。
    Memory(MemoryDevice),

    /// ファイルデバイス。
    File(FileDevice),
}
// FIXME: デフォルト実装は無くす（今はserdeのために必要）
impl Default for Device {
    fn default() -> Self {
        Device::Memory(MemoryDevice {
            id: String::new(),
            capacity: 0,
            seqno: 0,
            weight: Default::default(),
            server: String::new(),
        })
    }
}
impl Device {
    /// 要約情報を返す。
    pub fn to_summary(&self) -> DeviceSummary {
        DeviceSummary {
            id: self.id().to_owned(),
            server: self.server().cloned(),
            kind: self.kind(),
        }
    }

    /// 種類を返す。
    pub fn kind(&self) -> DeviceKind {
        match *self {
            Device::Virtual(_) => DeviceKind::Virtual,
            Device::Memory(_) => DeviceKind::Memory,
            Device::File(_) => DeviceKind::File,
        }
    }

    /// 仮想デバイスかどうかを判定する。
    pub fn is_virtual(&self) -> bool {
        if let Device::Virtual(_) = *self {
            true
        } else {
            false
        }
    }

    /// デバイスを保持しているサーバを返す。
    pub fn server(&self) -> Option<&ServerId> {
        match *self {
            Device::Memory(ref d) => Some(&d.server),
            Device::File(ref d) => Some(&d.server),
            _ => None,
        }
    }

    /// IDを返す。
    pub fn id(&self) -> &DeviceId {
        match *self {
            Device::Virtual(ref d) => &d.id,
            Device::Memory(ref d) => &d.id,
            Device::File(ref d) => &d.id,
        }
    }

    /// シーケンス番号を設定する。
    pub fn set_seqno(&mut self, seqno: u32) {
        match *self {
            Device::Virtual(ref mut d) => d.seqno = seqno,
            Device::Memory(ref mut d) => d.seqno = seqno,
            Device::File(ref mut d) => d.seqno = seqno,
        }
    }

    /// シーケンス番号を取得する。
    pub fn seqno(&self) -> u32 {
        match *self {
            Device::Virtual(ref d) => d.seqno,
            Device::Memory(ref d) => d.seqno,
            Device::File(ref d) => d.seqno,
        }
    }
}

/// 仮想デバイス。
///
/// 他のデバイス群をまとめるため構成要素。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualDevice {
    /// ID。
    pub id: DeviceId,

    /// シーケンス番号。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// オブジェクトの割当比率を決定する際の重み。
    #[serde(default)]
    pub weight: Weight,

    /// 子デバイス群。
    pub children: BTreeSet<DeviceId>,
    #[serde(default)]

    /// オブジェクトの割当方針。
    pub policy: SegmentAllocationPolicy,
}

/// メモリデバイス。
///
/// ここに保存されたオブジェクト群は永続化されない。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDevice {
    /// ID。
    pub id: DeviceId,

    /// シーケンス番号。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// オブジェクトの割当比率を決定する際の重み。
    #[serde(default)]
    pub weight: Weight,

    /// デバイスを保持しているサーバ。
    pub server: ServerId,

    /// 容量（バイト単位）。
    pub capacity: u64,
}
impl MemoryDevice {
    /// 重みを返す。
    pub fn weight(&self) -> u64 {
        self.weight.calculate(self.capacity)
    }
}

/// ファイルデバイス。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDevice {
    /// ID。
    pub id: DeviceId,

    /// シーケンス番号。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// オブジェクトの割当比率を決定する際の重み。
    #[serde(default)]
    pub weight: Weight,

    /// デバイスを保持しているサーバ。
    pub server: ServerId,

    /// 容量（バイト単位）。
    ///
    /// `0`が指定された場合には、ディスクの空き容量の99%の値が使用される。
    #[serde(default)]
    pub capacity: u64,

    /// ファイルパス。
    pub filepath: PathBuf,
}
impl FileDevice {
    /// 重みを返す。
    pub fn weight(&self) -> u64 {
        self.weight.calculate(self.capacity)
    }

    /// キャパシティを返す。
    pub fn capacity(&self) -> Result<u64> {
        if self.capacity == 0 {
            // for this case,
            // assign 99% of available space of the device

            // FIXME: `0`ではなくて明示的な省略を用意する
            // FIXME: エラーハンドリング（そもそもリクエスト受けた段階でバリデーションを行うべき）
            let dir = track_assert_some!(self.filepath.parent(), ErrorKind::InvalidInput);
            track!(std::fs::create_dir_all(dir).map_err(Error::from))?;

            let path = track_try_unwrap!(
                std::ffi::CString::new(dir.to_string_lossy().to_string()).map_err(Error::from)
            );

            let available_space: u64 = track!(calc_available_space(&path))?;
            Ok((available_space / 100) * 99)
        } else {
            Ok(self.capacity)
        }
    }
}

#[cfg(target_os = "macos")]
fn calc_available_space(path: &std::ffi::CString) -> Result<u64> {
    // on OS X,
    // statvfs's f_bsize != statfs's s_bsize
    let mut s: libc::statfs = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::statfs(path.as_ptr(), (&mut s) as _) };
    if result == 0 {
        Ok(u64::from(s.f_bsize) * s.f_bavail)
    } else {
        track!(Err(Error::from(std::io::Error::last_os_error())))
    }
}
#[cfg(not(target_os = "macos"))]
fn calc_available_space(path: &std::ffi::CString) -> Result<u64> {
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::statvfs(path.as_ptr(), (&mut s) as _) };
    if result == 0 {
        Ok(s.f_bsize * s.f_bavail)
    } else {
        track!(Err(Error::from(std::io::Error::last_os_error())))
    }
}

/// デバイスの重み。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weight {
    /// 自動計算。
    Auto,

    /// 絶対値の重み。
    Absolute(u64),

    /// 相対値の重み。
    Relative(f64),
}
impl Weight {
    /// 絶対値を重みを計算する。
    pub fn calculate(&self, base: u64) -> u64 {
        match *self {
            Weight::Auto => base,
            Weight::Absolute(v) => v,
            Weight::Relative(r) => (base as f64 * r) as u64,
        }
    }
}
impl Default for Weight {
    fn default() -> Self {
        Weight::Auto
    }
}

/// セグメントの割当方針。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentAllocationPolicy {
    /// 可能であれば、同じセグメント内のノード群には別々のデバイスを割り当てる。
    #[serde(rename = "SCATTER_IF_POSSIBLE")]
    ScatterIfPossible = 0,

    /// 同じセグメント内のノード群には別々のデバイスを割り当てる（無理ならエラーとなる）。
    #[serde(rename = "SCATTER")]
    Scatter = 1,

    /// 割当の際に「同じセグメントかどうか」を考慮せずに、個々のノードを完全に独立に割当てる。
    #[serde(rename = "NEUTRAL")]
    Neutral = 2,

    /// 同じセグメント内のノード群は同一のデバイスを割り当てる。
    #[serde(rename = "GATHER")]
    Gather = 3,
}
impl Default for SegmentAllocationPolicy {
    fn default() -> Self {
        SegmentAllocationPolicy::ScatterIfPossible
    }
}
