//! バケツ関連のエンティティ定義。
use std::cmp;

use entity::device::DeviceId;

// FIXME: 構造体に置き換える
/// バケツのID。
pub type BucketId = String;

/// バケツの内容の要約。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BucketSummary {
    /// バケツのID。
    pub id: BucketId,

    /// バケツの種類。
    #[serde(rename = "type")]
    pub kind: BucketKind,

    /// バケツが使用しているデバイス。
    pub device: DeviceId,
}

/// バケツの種類。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BucketKind {
    /// メタデータ用バケツ。
    Metadata,

    /// 複製による冗長化を行うバケツ。
    Replicated,

    /// ErasureCodingによる冗長化を行うバケツ。
    Dispersed,
}
impl Default for BucketKind {
    fn default() -> Self {
        BucketKind::Dispersed
    }
}

/// バケツ。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Bucket {
    /// メタデータ用バケツ。
    Metadata(MetadataBucket),

    /// 複製による冗長化を行うバケツ。
    Replicated(ReplicatedBucket),

    /// ErasureCodingによる冗長化を行うバケツ。
    Dispersed(DispersedBucket),
}
// FIXME: デフォルト実装は無くす（今はserdeのために必要）
impl Default for Bucket {
    fn default() -> Self {
        Bucket::Metadata(MetadataBucket {
            id: String::new(),
            seqno: 0,
            device: String::new(),
            segment_count: 0,
            tolerable_faults: 0,
        })
    }
}
impl Bucket {
    /// 必要ならセグメントカウントを自動計算して設定する.
    pub fn fix_segment_count(&mut self, physical_device_count: usize) {
        if self.segment_count() != 0 {
            return;
        }

        let average_segments_per_device = if let Bucket::Metadata(_) = *self {
            5
        } else {
            10
        };
        let segment_count =
            average_segments_per_device * physical_device_count / self.device_group_size() as usize;
        self.set_segment_count(cmp::max(1, segment_count) as u16);
    }

    /// バケツのセグメント数を設定する。
    pub fn set_segment_count(&mut self, count: u16) {
        match *self {
            Bucket::Metadata(ref mut b) => b.segment_count = u32::from(count),
            Bucket::Replicated(ref mut b) => b.segment_count = u32::from(count),
            Bucket::Dispersed(ref mut b) => b.segment_count = u32::from(count),
        }
    }

    /// バケツのセグメント数を返す。
    pub fn segment_count(&self) -> u16 {
        // FIXME: キャストではなく生成時にバリデーションする
        match *self {
            Bucket::Metadata(ref b) => b.segment_count as u16,
            Bucket::Replicated(ref b) => b.segment_count as u16,
            Bucket::Dispersed(ref b) => b.segment_count as u16,
        }
    }

    /// バケツのデバイスグループサイズ（i.e., Raftのクラスタサイズ）を返す。
    pub fn device_group_size(&self) -> u8 {
        // FIXME: キャストではなく生成時にバリデーションする
        match *self {
            Bucket::Metadata(ref b) => (b.tolerable_faults * 2 + 1) as u8,
            Bucket::Replicated(ref b) => (b.tolerable_faults * 2 + 1) as u8,
            Bucket::Dispersed(ref b) => {
                let raft_cluster_size = (b.tolerable_faults * 2 + 1) as u8;
                let ec_fragment_count = (b.tolerable_faults + b.data_fragment_count) as u8;
                cmp::max(raft_cluster_size, ec_fragment_count)
            }
        }
    }

    /// 対応する`BucketSummary`を返す。
    pub fn to_summary(&self) -> BucketSummary {
        BucketSummary {
            id: self.id().to_owned(),
            kind: self.kind(),
            device: self.device().to_owned(),
        }
    }

    /// バケツの種類を返す。
    pub fn kind(&self) -> BucketKind {
        match *self {
            Bucket::Metadata(_) => BucketKind::Metadata,
            Bucket::Replicated(_) => BucketKind::Replicated,
            Bucket::Dispersed(_) => BucketKind::Dispersed,
        }
    }

    /// バケツに紐付いているデバイスを返す。
    pub fn device(&self) -> &DeviceId {
        match *self {
            Bucket::Metadata(ref b) => &b.device,
            Bucket::Replicated(ref b) => &b.device,
            Bucket::Dispersed(ref b) => &b.device,
        }
    }

    /// バケツのIDを返す。
    pub fn id(&self) -> &BucketId {
        match *self {
            Bucket::Metadata(ref b) => &b.id,
            Bucket::Replicated(ref b) => &b.id,
            Bucket::Dispersed(ref b) => &b.id,
        }
    }

    /// バケツのシーケンス番号を設定する。
    pub fn set_seqno(&mut self, seqno: u32) {
        match *self {
            Bucket::Metadata(ref mut b) => b.seqno = seqno,
            Bucket::Replicated(ref mut b) => b.seqno = seqno,
            Bucket::Dispersed(ref mut b) => b.seqno = seqno,
        }
    }

    /// バケツのシーケンス番号を返す。
    pub fn seqno(&self) -> u32 {
        match *self {
            Bucket::Metadata(ref b) => b.seqno,
            Bucket::Replicated(ref b) => b.seqno,
            Bucket::Dispersed(ref b) => b.seqno,
        }
    }
}

/// メタデータ用のバケツ。
///
/// 他のバケツとは異なり、オブジェクトのデータは全てメモリ上に保持されるため、
/// PUT/GETは高速だが、メモリ負荷が高い。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataBucket {
    /// バケツのID。
    pub id: BucketId,

    /// バケツのシーケンス番号（登録番号）。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// バケツが使用するデバイス。
    pub device: DeviceId,

    /// バケツのセグメント数。
    #[serde(default)]
    pub segment_count: u32,

    /// 故障耐性数。
    pub tolerable_faults: u32,
}

/// 複製による冗長化を行うバケツ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicatedBucket {
    /// バケツのID。
    pub id: BucketId,

    /// バケツのシーケンス番号（登録番号）。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// バケツが使用するデバイス。
    pub device: DeviceId,

    /// バケツのセグメント数。
    #[serde(default)]
    pub segment_count: u32,

    /// 故障耐性数。
    ///
    /// `tolerable_faults + 1`が複製の数となる。
    pub tolerable_faults: u32,
}

/// ErasureCodingによる冗長化を行うバケツ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispersedBucket {
    /// バケツのID。
    pub id: BucketId,

    /// バケツのシーケンス番号（登録番号）。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// バケツが使用するデバイス。
    pub device: DeviceId,

    /// バケツのセグメント数。
    #[serde(default)]
    pub segment_count: u32,

    /// 故障耐性数。
    ///
    /// ErasureCodingのパリティフラグメント数でもある。
    pub tolerable_faults: u32,

    /// ErasureCodingのデータフラグメント数。
    pub data_fragment_count: u32,
}
