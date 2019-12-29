//! MDS系RPCのスキーマ定義。
use fibers_rpc::{Call, Cast, ProcedureId};
use std::ops::Range;
use std::time::Duration;

use consistency::ReadConsistency;
use entity::node::{LocalNodeId, RemoteNodeId};
use entity::object::{
    DeleteObjectsByPrefixSummary, Metadata, ObjectId, ObjectPrefix, ObjectSummary, ObjectVersion,
};
use expect::Expect;
use protobuf::schema::mds::{
    GetLatestVersionRequestDecoder, GetLatestVersionRequestEncoder, GetLeaderRequestDecoder,
    GetLeaderRequestEncoder, GetLeaderResponseDecoder, GetLeaderResponseEncoder,
    ListObjectsRequestDecoder, ListObjectsRequestEncoder, MaybeMetadataResponseDecoder,
    MaybeMetadataResponseEncoder, ObjectCountRequestDecoder, ObjectCountRequestEncoder,
    ObjectCountResponseDecoder, ObjectCountResponseEncoder, ObjectRequestDecoder,
    ObjectRequestEncoder, PrefixRequestDecoder, PrefixRequestEncoder, PutObjectRequestDecoder,
    PutObjectRequestEncoder, PutObjectResponseDecoder, PutObjectResponseEncoder,
    RangeRequestDecoder, RangeRequestEncoder, RecommendToLeaderRequestDecoder,
    RecommendToLeaderRequestEncoder, VersionRequestDecoder, VersionRequestEncoder,
};
use protobuf::schema::object::{
    DeleteObjectsByPrefixSummaryResponseDecoder, DeleteObjectsByPrefixSummaryResponseEncoder,
    MaybeObjectSummaryResponseDecoder, MaybeObjectSummaryResponseEncoder,
    MaybeObjectVersionResponseDecoder, MaybeObjectVersionResponseEncoder,
    ObjectSummarySequenceResponseDecoder, ObjectSummarySequenceResponseEncoder,
};
use Result;

/// Raftのリーダ取得RPC。
#[derive(Debug)]
pub struct GetLeaderRpc;
impl Call for GetLeaderRpc {
    const ID: ProcedureId = ProcedureId(0x0007_0000);
    const NAME: &'static str = "frugalos.mds.leader.get";

    type Req = GetLeaderRequest;
    type ReqDecoder = GetLeaderRequestDecoder;
    type ReqEncoder = GetLeaderRequestEncoder;

    type Res = Result<RemoteNodeId>;
    type ResDecoder = GetLeaderResponseDecoder;
    type ResEncoder = GetLeaderResponseEncoder;
}

/// リーダ推薦（再選挙）RPC。
#[derive(Debug)]
pub struct RecommendToLeaderRpc;
impl Cast for RecommendToLeaderRpc {
    const ID: ProcedureId = ProcedureId(0x0007_0001);
    const NAME: &'static str = "frugalos.mds.leader.recommend";

    type Notification = RecommendToLeaderRequest;
    type Decoder = RecommendToLeaderRequestDecoder;
    type Encoder = RecommendToLeaderRequestEncoder;
}

/// オブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsRpc;
impl Call for ListObjectsRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0000);
    const NAME: &'static str = "frugalos.mds.object.list";

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

/// オブジェクト取得RPC。
#[derive(Debug)]
pub struct GetObjectRpc;
impl Call for GetObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0001);
    const NAME: &'static str = "frugalos.mds.object.get";

    type Req = ObjectRequest;
    type ReqDecoder = ObjectRequestDecoder;
    type ReqEncoder = ObjectRequestEncoder;

    type Res = Result<Option<Metadata>>;
    type ResDecoder = MaybeMetadataResponseDecoder;
    type ResEncoder = MaybeMetadataResponseEncoder;
}

/// オブジェクト存在確認RPC。
#[derive(Debug)]
pub struct HeadObjectRpc;
impl Call for HeadObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0002);
    const NAME: &'static str = "frugalos.mds.object.head";

    type Req = ObjectRequest;
    type ReqDecoder = ObjectRequestDecoder;
    type ReqEncoder = ObjectRequestEncoder;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = MaybeObjectVersionResponseDecoder;
    type ResEncoder = MaybeObjectVersionResponseEncoder;
}

/// オブジェクト保存RPC。
#[derive(Debug)]
pub struct PutObjectRpc;
impl Call for PutObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0003);
    const NAME: &'static str = "frugalos.mds.object.put";

    type Req = PutObjectRequest;
    type ReqDecoder = PutObjectRequestDecoder;
    type ReqEncoder = PutObjectRequestEncoder;

    type Res = Result<(ObjectVersion, Option<ObjectVersion>)>;
    type ResDecoder = PutObjectResponseDecoder;
    type ResEncoder = PutObjectResponseEncoder;
}

/// オブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectRpc;
impl Call for DeleteObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0004);
    const NAME: &'static str = "frugalos.mds.object.delete";

    type Req = ObjectRequest;
    type ReqDecoder = ObjectRequestDecoder;
    type ReqEncoder = ObjectRequestEncoder;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = MaybeObjectVersionResponseDecoder;
    type ResEncoder = MaybeObjectVersionResponseEncoder;
}

/// 最新バージョン取得RPC。
#[derive(Debug)]
pub struct GetLatestVersionRpc;
impl Call for GetLatestVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0005);
    const NAME: &'static str = "frugalos.mds.object.latest_version";

    type Req = GetLatestVersionRequest;
    type ReqDecoder = GetLatestVersionRequestDecoder;
    type ReqEncoder = GetLatestVersionRequestEncoder;

    type Res = Result<Option<ObjectSummary>>;
    type ResDecoder = MaybeObjectSummaryResponseDecoder;
    type ResEncoder = MaybeObjectSummaryResponseEncoder;
}

/// バージョン指定による削除RPC。
#[derive(Debug)]
pub struct DeleteObjectByVersionRpc;
impl Call for DeleteObjectByVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0006);
    const NAME: &'static str = "frugalos.mds.object.delete_by_version";

    type Req = VersionRequest;
    type ReqDecoder = VersionRequestDecoder;
    type ReqEncoder = VersionRequestEncoder;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = MaybeObjectVersionResponseDecoder;
    type ResEncoder = MaybeObjectVersionResponseEncoder;
}

/// バージョン範囲指定による削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByRangeRpc;
impl Call for DeleteObjectsByRangeRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0007);
    const NAME: &'static str = "frugalos.mds.object.delete_by_range";

    type Req = RangeRequest;
    type ReqDecoder = RangeRequestDecoder;
    type ReqEncoder = RangeRequestEncoder;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = ObjectSummarySequenceResponseDecoder;
    type ResEncoder = ObjectSummarySequenceResponseEncoder;
}

/// 格納済みオブジェクト数取得RPC。
#[derive(Debug)]
pub struct GetObjectCountRpc;
impl Call for GetObjectCountRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0008);
    const NAME: &'static str = "frugalos.mds.object.count";

    type Req = ObjectCountRequest;
    type ReqDecoder = ObjectCountRequestDecoder;
    type ReqEncoder = ObjectCountRequestEncoder;

    type Res = Result<u64>;
    type ResDecoder = ObjectCountResponseDecoder;
    type ResEncoder = ObjectCountResponseEncoder;
}

/// 接頭辞削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByPrefixRpc;
impl Call for DeleteObjectsByPrefixRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0009);
    const NAME: &'static str = "frugalos.mds.object.delete_by_prefix";

    type Req = PrefixRequest;
    type ReqDecoder = PrefixRequestDecoder;
    type ReqEncoder = PrefixRequestEncoder;

    type Res = Result<DeleteObjectsByPrefixSummary>;
    type ResDecoder = DeleteObjectsByPrefixSummaryResponseDecoder;
    type ResEncoder = DeleteObjectsByPrefixSummaryResponseEncoder;
}

/// 接頭辞指定でのオブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsByPrefixRpc;
impl Call for ListObjectsByPrefixRpc {
    const ID: ProcedureId = ProcedureId(0x0008_000a);
    const NAME: &'static str = "frugalos.mds.object.list_by_prefix";

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

/// リーダー取得要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaderRequest {
    pub node_id: LocalNodeId,
}

/// リーダー再選挙要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendToLeaderRequest {
    pub node_id: LocalNodeId,
}

/// 最新バージョン取得要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestVersionRequest {
    pub node_id: LocalNodeId,
}

/// オブジェクト単位の要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRequest {
    pub node_id: LocalNodeId,
    pub object_id: ObjectId,
    pub expect: Expect,
    pub consistency: Option<ReadConsistency>,
}

/// オブジェクト一覧の要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsRequest {
    pub node_id: LocalNodeId,
    pub consistency: ReadConsistency,
}

/// オブジェクトカウントの要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectCountRequest {
    pub node_id: LocalNodeId,
    pub consistency: ReadConsistency,
}

/// バージョン単位の要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRequest {
    pub node_id: LocalNodeId,
    pub object_version: ObjectVersion,
}

/// バージョン範囲指定の要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeRequest {
    pub node_id: LocalNodeId,
    pub targets: Range<ObjectVersion>,
}

/// オブジェクトの接頭辞単位の要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixRequest {
    pub node_id: LocalNodeId,
    pub prefix: ObjectPrefix,
}

/// オブジェクト保存要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectRequest {
    pub node_id: LocalNodeId,
    pub object_id: ObjectId,
    pub metadata: Vec<u8>,
    pub expect: Expect,
    pub put_content_timeout: Duration,
}
