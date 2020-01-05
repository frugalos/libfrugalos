//! frugalosの公開API系RPCのスキーマ定義。
use fibers_rpc::{Call, ProcedureId};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};
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
use protobuf::repair::{RepairConfigDecoder, RepairConfigEncoder};
use protobuf::schema::frugalos::{
    CountFragmentsRequestDecoder, CountFragmentsRequestEncoder, CountFragmentsResponseDecoder,
    CountFragmentsResponseEncoder, DeleteObjectSetFromDeviceRequestDecoder,
    DeleteObjectSetFromDeviceRequestEncoder, DeleteObjectSetFromDeviceResponseDecoder,
    DeleteObjectSetFromDeviceResponseEncoder, GetObjectResponseDecoder, GetObjectResponseEncoder,
    HeadObjectRequestDecoder, HeadObjectRequestEncoder, ListObjectsRequestDecoder,
    ListObjectsRequestEncoder, ObjectRequestDecoder, ObjectRequestEncoder, PrefixRequestDecoder,
    PrefixRequestEncoder, PutObjectRequestDecoder, PutObjectRequestEncoder,
    PutObjectResponseDecoder, PutObjectResponseEncoder, RangeRequestDecoder, RangeRequestEncoder,
    SegmentRequestDecoder, SegmentRequestEncoder, SetRepairConfigResponseDecoder,
    SetRepairConfigResponseEncoder, StopResponseDecoder, StopResponseEncoder,
    TakeSnapshotResponseDecoder, TakeSnapshotResponseEncoder, VersionRequestDecoder,
    VersionRequestEncoder,
};
use protobuf::schema::object::{
    DeleteObjectsByPrefixSummaryResponseDecoder, DeleteObjectsByPrefixSummaryResponseEncoder,
    MaybeObjectSummaryResponseDecoder, MaybeObjectSummaryResponseEncoder,
    MaybeObjectVersionResponseDecoder, MaybeObjectVersionResponseEncoder,
    ObjectSummarySequenceResponseDecoder, ObjectSummarySequenceResponseEncoder,
};
use repair::RepairConfig;
use Result;

/// オブジェクト取得RPC。
#[derive(Debug)]
pub struct GetObjectRpc;
impl Call for GetObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0000);
    const NAME: &'static str = "frugalos.object.get";

    type Req = ObjectRequest;
    type ReqDecoder = ObjectRequestDecoder;
    type ReqEncoder = ObjectRequestEncoder;

    // FIXME: データが巨大になる可能性があるのでbincodeはやめる
    type Res = Result<Option<(ObjectVersion, Vec<u8>)>>;
    type ResDecoder = GetObjectResponseDecoder;
    type ResEncoder = GetObjectResponseEncoder;
}

/// オブジェクト存在確認RPC。
#[derive(Debug)]
pub struct HeadObjectRpc;
impl Call for HeadObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0001);
    const NAME: &'static str = "frugalos.object.head";

    type Req = HeadObjectRequest;
    type ReqDecoder = HeadObjectRequestDecoder;
    type ReqEncoder = HeadObjectRequestEncoder;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = MaybeObjectVersionResponseDecoder;
    type ResEncoder = MaybeObjectVersionResponseEncoder;
}

/// オブジェクト保存RPC。
#[derive(Debug)]
pub struct PutObjectRpc;
impl Call for PutObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0002);
    const NAME: &'static str = "frugalos.object.put";

    // FIXME: データが巨大になる可能性があるのでbincodeはやめる
    type Req = PutObjectRequest;
    type ReqDecoder = PutObjectRequestDecoder;
    type ReqEncoder = PutObjectRequestEncoder;

    type Res = Result<(ObjectVersion, bool)>;
    type ResDecoder = PutObjectResponseDecoder;
    type ResEncoder = PutObjectResponseEncoder;
}

/// オブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectRpc;
impl Call for DeleteObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0003);
    const NAME: &'static str = "frugalos.object.delete";

    type Req = ObjectRequest;
    type ReqDecoder = ObjectRequestDecoder;
    type ReqEncoder = ObjectRequestEncoder;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = MaybeObjectVersionResponseDecoder;
    type ResEncoder = MaybeObjectVersionResponseEncoder;
}

/// オブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsRpc;
impl Call for ListObjectsRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0004);
    const NAME: &'static str = "frugalos.object.list";

    type Req = ListObjectsRequest;
    type ReqDecoder = ListObjectsRequestDecoder;
    type ReqEncoder = ListObjectsRequestEncoder;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = ObjectSummarySequenceResponseDecoder;
    type ResEncoder = ObjectSummarySequenceResponseEncoder;

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
    type ReqDecoder = SegmentRequestDecoder;
    type ReqEncoder = SegmentRequestEncoder;

    type Res = Result<Option<ObjectSummary>>;
    type ResDecoder = MaybeObjectSummaryResponseDecoder;
    type ResEncoder = MaybeObjectSummaryResponseEncoder;
}

/// バージョン指定でのオブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectByVersionRpc;
impl Call for DeleteObjectByVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0006);
    const NAME: &'static str = "frugalos.object.delete_by_version";

    type Req = VersionRequest;
    type ReqDecoder = VersionRequestDecoder;
    type ReqEncoder = VersionRequestEncoder;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = MaybeObjectVersionResponseDecoder;
    type ResEncoder = MaybeObjectVersionResponseEncoder;
}

/// バージョンの範囲指定でのオブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByRangeRpc;
impl Call for DeleteObjectsByRangeRpc {
    const ID: ProcedureId = ProcedureId(0x0009_0007);
    const NAME: &'static str = "frugalos.object.delete_by_range";

    type Req = RangeRequest;
    type ReqDecoder = RangeRequestDecoder;
    type ReqEncoder = RangeRequestEncoder;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = ObjectSummarySequenceResponseDecoder;
    type ResEncoder = ObjectSummarySequenceResponseEncoder;

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
    type ReqDecoder = PrefixRequestDecoder;
    type ReqEncoder = PrefixRequestEncoder;

    type Res = Result<DeleteObjectsByPrefixSummary>;
    type ResDecoder = DeleteObjectsByPrefixSummaryResponseDecoder;
    type ResEncoder = DeleteObjectsByPrefixSummaryResponseEncoder;
}

/// An RPC for deleting objects physically.
#[derive(Debug)]
pub struct DeleteObjectSetFromDeviceRpc;
impl Call for DeleteObjectSetFromDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0009_000a);
    const NAME: &'static str = "frugalos.object.delete_object_set_from_device";

    type Req = DeleteObjectSetFromDeviceRequest;
    type ReqDecoder = DeleteObjectSetFromDeviceRequestDecoder;
    type ReqEncoder = DeleteObjectSetFromDeviceRequestEncoder;

    type Res = Result<()>;
    type ResDecoder = DeleteObjectSetFromDeviceResponseDecoder;
    type ResEncoder = DeleteObjectSetFromDeviceResponseEncoder;
}

/// 接頭辞指定でのオブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsByPrefixRpc;
impl Call for ListObjectsByPrefixRpc {
    const ID: ProcedureId = ProcedureId(0x0009_000b);
    const NAME: &'static str = "frugalos.object.list_by_prefix";

    type Req = PrefixRequest;
    type ReqDecoder = PrefixRequestDecoder;
    type ReqEncoder = PrefixRequestEncoder;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = ObjectSummarySequenceResponseDecoder;
    type ResEncoder = ObjectSummarySequenceResponseEncoder;

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
    type ReqDecoder = CountFragmentsRequestDecoder;
    type ReqEncoder = CountFragmentsRequestEncoder;

    type Res = Result<Option<FragmentsSummary>>;
    type ResDecoder = CountFragmentsResponseDecoder;
    type ResEncoder = CountFragmentsResponseEncoder;
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

/// オブジェクト一覧要求。
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectsRequest {
    pub bucket_id: BucketId,
    pub segment: u16,
    pub consistency: ReadConsistency,
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
    type ReqDecoder = EmptyMessageDecoder;
    type ReqEncoder = EmptyMessageEncoder;

    type Res = Result<()>;
    type ResDecoder = StopResponseDecoder;
    type ResEncoder = StopResponseEncoder;
}

/// スナップショット取得RPC。
#[derive(Debug)]
pub struct TakeSnapshotRpc;
impl Call for TakeSnapshotRpc {
    const ID: ProcedureId = ProcedureId(0x000a_0001);
    const NAME: &'static str = "frugalos.ctrl.take_snapshot";

    type Req = ();
    type ReqDecoder = EmptyMessageDecoder;
    type ReqEncoder = EmptyMessageEncoder;

    type Res = Result<()>;
    type ResDecoder = TakeSnapshotResponseDecoder;
    type ResEncoder = TakeSnapshotResponseEncoder;
}

/// An RPC for changing configuration of repair functionality.
#[derive(Debug)]
pub struct SetRepairConfigRpc;
impl Call for SetRepairConfigRpc {
    const ID: ProcedureId = ProcedureId(0x000a_0002);
    const NAME: &'static str = "frugalos.ctrl.set_repair_config";

    type Req = RepairConfig;
    type ReqEncoder = RepairConfigEncoder;
    type ReqDecoder = RepairConfigDecoder;

    type Res = Result<()>;
    type ResEncoder = SetRepairConfigResponseEncoder;
    type ResDecoder = SetRepairConfigResponseDecoder;
}
