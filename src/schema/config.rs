//! 構成管理系RPCのスキーマ定義。
use fibers_rpc::{Call, ProcedureId};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};
use std::net::SocketAddr;

use entity::bucket::{Bucket, BucketId, BucketSummary};
use entity::device::{Device, DeviceId, DeviceSummary};
use entity::server::{Server, ServerId, ServerSummary};
use protobuf::entity::bucket::{BucketDecoder, BucketEncoder, BucketIdDecoder, BucketIdEncoder};
use protobuf::entity::device::{DeviceDecoder, DeviceEncoder, DeviceIdDecoder, DeviceIdEncoder};
use protobuf::entity::server::{ServerDecoder, ServerEncoder, ServerIdDecoder, ServerIdEncoder};
use protobuf::schema::config::{
    DeleteBucketResponseDecoder, DeleteBucketResponseEncoder, DeleteDeviceResponseDecoder,
    DeleteDeviceResponseEncoder, DeleteServerResponseDecoder, DeleteServerResponseEncoder,
    GetBucketResponseDecoder, GetBucketResponseEncoder, GetDeviceResponseDecoder,
    GetDeviceResponseEncoder, GetLeaderResponseDecoder, GetLeaderResponseEncoder,
    GetServerResponseDecoder, GetServerResponseEncoder, ListBucketsResponseDecoder,
    ListBucketsResponseEncoder, ListDevicesResponseDecoder, ListDevicesResponseEncoder,
    ListServersResponseDecoder, ListServersResponseEncoder, PutBucketResponseDecoder,
    PutBucketResponseEncoder, PutDeviceResponseDecoder, PutDeviceResponseEncoder,
    PutServerResponseDecoder, PutServerResponseEncoder,
};
use Result;

/// サーバ一覧取得RPC。
#[derive(Debug)]
pub struct ListServersRpc;
impl Call for ListServersRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0000);
    const NAME: &'static str = "frugalos.config.server.list";

    type Req = ();
    type ReqDecoder = EmptyMessageDecoder;
    type ReqEncoder = EmptyMessageEncoder;

    type Res = Result<Vec<ServerSummary>>;
    type ResDecoder = ListServersResponseDecoder;
    type ResEncoder = ListServersResponseEncoder;
}

/// サーバ情報取得RPC。
#[derive(Debug)]
pub struct GetServerRpc;
impl Call for GetServerRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0001);
    const NAME: &'static str = "frugalos.config.server.get";

    type Req = ServerId;
    type ReqDecoder = ServerIdDecoder;
    type ReqEncoder = ServerIdEncoder;

    type Res = Result<Option<Server>>;
    type ResDecoder = GetServerResponseDecoder;
    type ResEncoder = GetServerResponseEncoder;
}

/// サーバ登録RPC。
#[derive(Debug)]
pub struct PutServerRpc;
impl Call for PutServerRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0002);
    const NAME: &'static str = "frugalos.config.server.put";

    type Req = Server;
    type ReqDecoder = ServerDecoder;
    type ReqEncoder = ServerEncoder;

    type Res = Result<Server>;
    type ResDecoder = PutServerResponseDecoder;
    type ResEncoder = PutServerResponseEncoder;
}

/// サーバ削除RPC。
#[derive(Debug)]
pub struct DeleteServerRpc;
impl Call for DeleteServerRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0003);
    const NAME: &'static str = "frugalos.config.server.delete";

    type Req = ServerId;
    type ReqDecoder = ServerIdDecoder;
    type ReqEncoder = ServerIdEncoder;

    type Res = Result<Option<Server>>;
    type ResDecoder = DeleteServerResponseDecoder;
    type ResEncoder = DeleteServerResponseEncoder;
}

/// デバイス一覧取得RPC。
#[derive(Debug)]
pub struct ListDevicesRpc;
impl Call for ListDevicesRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0000);
    const NAME: &'static str = "frugalos.config.device.list";

    type Req = ();
    type ReqDecoder = EmptyMessageDecoder;
    type ReqEncoder = EmptyMessageEncoder;

    type Res = Result<Vec<DeviceSummary>>;
    type ResDecoder = ListDevicesResponseDecoder;
    type ResEncoder = ListDevicesResponseEncoder;
}

/// デバイス情報取得RPC。
#[derive(Debug)]
pub struct GetDeviceRpc;
impl Call for GetDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0001);
    const NAME: &'static str = "frugalos.config.device.get";

    type Req = DeviceId;
    type ReqDecoder = DeviceIdDecoder;
    type ReqEncoder = DeviceIdEncoder;

    type Res = Result<Option<Device>>;
    type ResDecoder = GetDeviceResponseDecoder;
    type ResEncoder = GetDeviceResponseEncoder;
}

/// デバイス登録RPC。
#[derive(Debug)]
pub struct PutDeviceRpc;
impl Call for PutDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0002);
    const NAME: &'static str = "frugalos.config.device.put";

    type Req = Device;
    type ReqDecoder = DeviceDecoder;
    type ReqEncoder = DeviceEncoder;

    type Res = Result<Device>;
    type ResDecoder = PutDeviceResponseDecoder;
    type ResEncoder = PutDeviceResponseEncoder;
}

/// デバイス削除RPC。
#[derive(Debug)]
pub struct DeleteDeviceRpc;
impl Call for DeleteDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0003);
    const NAME: &'static str = "frugalos.config.device.delete";

    type Req = DeviceId;
    type ReqDecoder = DeviceIdDecoder;
    type ReqEncoder = DeviceIdEncoder;

    type Res = Result<Option<Device>>;
    type ResDecoder = DeleteDeviceResponseDecoder;
    type ResEncoder = DeleteDeviceResponseEncoder;
}

/// バケツ一覧取得RPC。
#[derive(Debug)]
pub struct ListBucketsRpc;
impl Call for ListBucketsRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0000);
    const NAME: &'static str = "frugalos.config.bucket.list";

    type Req = ();
    type ReqDecoder = EmptyMessageDecoder;
    type ReqEncoder = EmptyMessageEncoder;

    type Res = Result<Vec<BucketSummary>>;
    type ResDecoder = ListBucketsResponseDecoder;
    type ResEncoder = ListBucketsResponseEncoder;
}

/// バケツ情報取得RPC。
#[derive(Debug)]
pub struct GetBucketRpc;
impl Call for GetBucketRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0001);
    const NAME: &'static str = "frugalos.config.bucket.get";

    type Req = BucketId;
    type ReqDecoder = BucketIdDecoder;
    type ReqEncoder = BucketIdEncoder;

    type Res = Result<Option<Bucket>>;
    type ResDecoder = GetBucketResponseDecoder;
    type ResEncoder = GetBucketResponseEncoder;
}

/// バケツ登録RPC。
#[derive(Debug)]
pub struct PutBucketRpc;
impl Call for PutBucketRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0002);
    const NAME: &'static str = "frugalos.config.bucket.put";

    type Req = Bucket;
    type ReqDecoder = BucketDecoder;
    type ReqEncoder = BucketEncoder;

    type Res = Result<Bucket>;
    type ResDecoder = PutBucketResponseDecoder;
    type ResEncoder = PutBucketResponseEncoder;
}

/// バケツ削除RPC。
#[derive(Debug)]
pub struct DeleteBucketRpc;
impl Call for DeleteBucketRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0003);
    const NAME: &'static str = "frugalos.config.bucket.delete";

    type Req = BucketId;
    type ReqDecoder = BucketIdDecoder;
    type ReqEncoder = BucketIdEncoder;

    type Res = Result<Option<Bucket>>;
    type ResDecoder = DeleteBucketResponseDecoder;
    type ResEncoder = DeleteBucketResponseEncoder;
}

/// Raftのリーダノード取得RPC。
// NOTE: リーダ選出中の場合にはserver側でwaitする
#[derive(Debug)]
pub struct GetLeaderRpc;
impl Call for GetLeaderRpc {
    const ID: ProcedureId = ProcedureId(0x0005_0000);
    const NAME: &'static str = "frugalos.config.leader.get";

    type Req = ();
    type ReqDecoder = EmptyMessageDecoder;
    type ReqEncoder = EmptyMessageEncoder;

    type Res = Result<SocketAddr>;
    type ResDecoder = GetLeaderResponseDecoder;
    type ResEncoder = GetLeaderResponseEncoder;
}
