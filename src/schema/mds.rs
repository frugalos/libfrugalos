//! MDS系RPCのスキーマ定義。
use bytecodec::bincode_codec::{BincodeDecoder, BincodeEncoder};
use fibers_rpc::{Call, Cast, ProcedureId};
use std::ops::Range;
use std::time::Duration;

use entity::node::{LocalNodeId, RemoteNodeId};
use entity::object::{
    DeleteObjectsByPrefixSummary, Metadata, ObjectId, ObjectPrefix, ObjectSummary, ObjectVersion,
};
use expect::Expect;
use Result;

/// Raftのリーダ取得RPC。
#[derive(Debug)]
pub struct GetLeaderRpc;
impl Call for GetLeaderRpc {
    const ID: ProcedureId = ProcedureId(0x0007_0000);
    const NAME: &'static str = "frugalos.mds.leader.get";

    type Req = LocalNodeId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<RemoteNodeId>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// リーダ推薦（再選挙）RPC。
#[derive(Debug)]
pub struct RecommendToLeaderRpc;
impl Cast for RecommendToLeaderRpc {
    const ID: ProcedureId = ProcedureId(0x0007_0001);
    const NAME: &'static str = "frugalos.mds.leader.recommend";

    type Notification = LocalNodeId;
    type Decoder = BincodeDecoder<Self::Notification>;
    type Encoder = BincodeEncoder<Self::Notification>;
}

/// オブジェクト一覧取得RPC。
#[derive(Debug)]
pub struct ListObjectsRpc;
impl Call for ListObjectsRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0000);
    const NAME: &'static str = "frugalos.mds.object.list";

    type Req = LocalNodeId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;

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
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Metadata>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト存在確認RPC。
#[derive(Debug)]
pub struct HeadObjectRpc;
impl Call for HeadObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0002);
    const NAME: &'static str = "frugalos.mds.object.head";

    type Req = ObjectRequest;
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
    const ID: ProcedureId = ProcedureId(0x0008_0003);
    const NAME: &'static str = "frugalos.mds.object.put";

    type Req = PutObjectRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<(ObjectVersion, Option<ObjectVersion>)>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト削除RPC。
#[derive(Debug)]
pub struct DeleteObjectRpc;
impl Call for DeleteObjectRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0004);
    const NAME: &'static str = "frugalos.mds.object.delete";

    type Req = ObjectRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// 最新バージョン取得RPC。
#[derive(Debug)]
pub struct GetLatestVersionRpc;
impl Call for GetLatestVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0005);
    const NAME: &'static str = "frugalos.mds.object.latest_version";

    type Req = LocalNodeId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バージョン指定による削除RPC。
#[derive(Debug)]
pub struct DeleteObjectByVersionRpc;
impl Call for DeleteObjectByVersionRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0006);
    const NAME: &'static str = "frugalos.mds.object.delete_by_version";

    type Req = VersionRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<ObjectVersion>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バージョン範囲指定による削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByRangeRpc;
impl Call for DeleteObjectsByRangeRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0007);
    const NAME: &'static str = "frugalos.mds.object.delete_by_range";

    type Req = RangeRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<ObjectSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// 格納済みオブジェクト数取得RPC。
#[derive(Debug)]
pub struct GetObjectCountRpc;
impl Call for GetObjectCountRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0008);
    const NAME: &'static str = "frugalos.mds.object.count";

    type Req = LocalNodeId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<u64>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// 接頭辞削除RPC。
#[derive(Debug)]
pub struct DeleteObjectsByPrefixRpc;
impl Call for DeleteObjectsByPrefixRpc {
    const ID: ProcedureId = ProcedureId(0x0008_0009);
    const NAME: &'static str = "frugalos.mds.object.delete_by_prefix";

    type Req = PrefixRequest;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<DeleteObjectsByPrefixSummary>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// オブジェクト単位の要求。
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRequest {
    pub node_id: LocalNodeId,
    pub object_id: ObjectId,
    pub expect: Expect,
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
