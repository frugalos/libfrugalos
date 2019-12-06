//! frugalosの公開API系RPCのスキーマ定義。
use bytecodec::bincode_codec::{BincodeDecoder, BincodeEncoder};
use fibers_rpc::{Call, ProcedureId};
use std::collections::BTreeSet;
use std::ops::Range;
use std::time::Duration;

use consistency::ReadConsistency;
use entity::bucket::BucketId;
use entity::device::DeviceId;
use entity::object::{
    DeleteObjectsByPrefixSummary, FragmentsSummary, ObjectId, ObjectPrefix, ObjectSummary,
    ObjectVersion,
};
use expect::Expect;
use multiplicity::MultiplicityConfig;
use repair::RepairConfig;
use Result;

/// オブジェクト取得RPC。
#[derive(Debug)]
pub struct GetObjectRpc;
impl Call for GetObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0000);
    const NAME: &'static str = "frugalos.object.get";

    type Req = ObjectRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    // FIXME: データが巨大になる可能性があるのでbincodeはやめる
    type Res = Result<Option<(ObjectVersion, Vec<u8>)>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト存在確認RPC。
#[derive(Debug)]
pub struct HeadObjectRpc;
impl Call for HeadObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0001);
    const NAME: &'static str = "frugalos.object.head";

    type Req = HeadObjectRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト保存RPC。
#[derive(Debug)]
pub struct PutObjectRpc;
impl Call for PutObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0002);
    const NAME: &'static str = "frugalos.object.put";

    // FIXME: データが巨大になる可能性があるのでbincodeはやめる
    type Req = PutObjectRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<(ObjectVersion, bool)>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectRpc;
impl Call for DeleteObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0003);
    const NAME: &'static str = "frugalos.object.delete";

    type Req = ObjectRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsRpc;
impl Call for ListObjectsRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0004);
    const NAME: &'static str = "frugalos.object.list";

    type Req = SegmentRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;

    fn enable_async_response(_: &Self::Res) -> bool {
        true
    }
}

/// 最新バージョン取得RPC。
#[derive(Debug)]
pub struct GetLatestVersionRpc;
impl Call for GetLatestVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0005);
    const NAME: &'static str = "frugalos.object.latest_version";

    type Req = SegmentRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バージョン指定でのオブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectByVersionRpc;
impl Call for DeleteObjectByVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0006);
    const NAME: &'static str = "frugalos.object.delete_by_version";

    type Req = VersionRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バージョンの範囲指定でのオブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByRangeRpc;
impl Call for DeleteObjectsByRangeRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0007);
    const NAME: &'static str = "frugalos.object.delete_by_range";

    type Req = RangeRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;

    /*
    NOTE:
    このメソッドがtrueを返すと、応答メッセージのencode/decodeは、
    スレッドプール内のスレッド上で行われることになり、
    future群のスケジューラスレッドの進行は阻害しない
     */
    fn enable_async_response(_: &Self::Res) -> bool {
        true
    }
}

/// 接頭辞削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByPrefixRpc;
impl Call for DeleteObjectsByPrefixRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0009);
    const NAME: &'static str = "frugalos.object.delete_by_prefix";

    type Req = PrefixRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<DeleteObjectsByPrefixSummary>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// An RPC for deleting objects physically.
#[derive(Debug)]
pub struct DeleteObjectSetFromDeviceRpc;
impl Call for DeleteObjectSetFromDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0009_000a);
    const NAME: &'static str = "frugalos.object.delete_object_set_from_device";

    type Req = DeleteObjectSetFromDeviceRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<()>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// 接頭辞指定でのオブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsByPrefixRpc;
impl Call for ListObjectsByPrefixRpc {
    const ID: ProcedureId = ProcedureId(0x0009_000b);
    const NAME: &'static str = "frugalos.object.list_by_prefix";

    type Req = PrefixRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;

    fn enable_async_response(_: &Self::Res) -> bool {
        true
    }
}

/// フラグメントカウントRPC。
#[derive(Debug)]
pub struct CountFragmentsRpc;
impl Call for CountFragmentsRpc {
    const ID: ProcedureId = ProcedureId(0x0009_000d);
    const NAME: &'static str = "frugalos.object.count_fragments";

    type Req = CountFragmentsRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<FragmentsSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト単位のRPC要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRequest {
    pub bucket_id: BucketId,
    pub object_id: ObjectId,
    pub deadline: Duration,
    pub expect: Expect,
    pub consistency: Option<ReadConsistency>,
}

/// フラグメントカウント RPC 要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CountFragmentsRequest {
    pub bucket_id: BucketId,
    pub object_id: ObjectId,
    pub deadline: Duration,
    pub expect: Expect,
    pub consistency: ReadConsistency,
}

/// オブジェクト単位の存在確認 RPC 要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadObjectRequest {
    pub bucket_id: BucketId,
    pub object_id: ObjectId,
    pub deadline: Duration,
    pub expect: Expect,
    pub consistency: ReadConsistency,
    /// ストレージ側にも問い合わせるかどうか
    pub check_storage: bool,
}

/// バージョン単位のRPC要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionRequest {
    pub bucket_id: BucketId,
    pub segment: u16,
    pub object_version: ObjectVersion,
    pub deadline: Duration,
}

/// バージョン範囲でのRPC要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RangeRequest {
    pub bucket_id: BucketId,
    pub segment: u16,
    pub targets: Range<ObjectVersion>,
    pub deadline: Duration,
}

/// オブジェクトの接頭辞単位でのRPC要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefixRequest {
    pub bucket_id: BucketId,
    pub prefix: ObjectPrefix,
    pub deadline: Duration,
}

/// オブジェクト保存要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectRequest {
    pub bucket_id: BucketId,
    pub object_id: ObjectId,
    pub content: Vec<u8>,
    pub deadline: Duration,
    pub expect: Expect,
    pub multiplicity_config: MultiplicityConfig,
}

/// セグメント単位でのRPC要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentRequest {
    pub bucket_id: BucketId,
    pub segment: u16,
}

/// This struct represents how to delete objects from a device at once.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectSetFromDeviceRequest {
    /// A bucket may own the objects.
    pub bucket_id: BucketId,

    /// A device may own the objects.
    pub device_id: DeviceId,

    /// The objects will be deleted.
    pub object_ids: BTreeSet<ObjectId>,
}

/// プロセス停止RPC。
#[derive(Debug)]
pub struct StopRpc;
impl Call for StopRpc {
    const ID: ProcedureId = ProcedureId(0x000a_0000);
    const NAME: &'static str = "frugalos.ctrl.stop";

    type Req = ();
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<()>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// スナップショット取得RPC。
#[derive(Debug)]
pub struct TakeSnapshotRpc;
impl Call for TakeSnapshotRpc {
    const ID: ProcedureId = ProcedureId(0x000a_0001);
    const NAME: &'static str = "frugalos.ctrl.take_snapshot";

    type Req = ();
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<()>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// An RPC for changing configuration of repair functionality.
#[derive(Debug)]
pub struct SetRepairConfigRpc;
impl Call for SetRepairConfigRpc {
    const ID: ProcedureId = ProcedureId(0x000a_0002);
    const NAME: &'static str = "frugalos.ctrl.set_repair_config";

    type Req = RepairConfig;
    type ReqEncoder = BincodeEncoder<Self::Req>;
    type ReqDecoder = BincodeDecoder<Self::Req>;

    type Res = Result<()>;
    type ResEncoder = BincodeEncoder<Self::Res>;
    type ResDecoder = BincodeDecoder<Self::Res>;
}
