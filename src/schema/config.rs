//! 構成管理系RPCのスキーマ定義。
use bytecodec::bincode_codec::{BincodeDecoder, BincodeEncoder};
use fibers_rpc::{Call, ProcedureId};
use std::net::SocketAddr;

use crate::entity::bucket::{Bucket, BucketId, BucketSummary};
use crate::entity::device::{Device, DeviceId, DeviceSummary};
use crate::entity::server::{Server, ServerId, ServerSummary};
use crate::Result;

/// サーバ一覧取得RPC。
#[derive(Debug)]
pub struct ListServersRpc;
impl Call for ListServersRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0000);
    const NAME: &'static str = "frugalos.config.server.list";

    type Req = ();
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<ServerSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// サーバ情報取得RPC。
#[derive(Debug)]
pub struct GetServerRpc;
impl Call for GetServerRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0001);
    const NAME: &'static str = "frugalos.config.server.get";

    type Req = ServerId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Server>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// サーバ登録RPC。
#[derive(Debug)]
pub struct PutServerRpc;
impl Call for PutServerRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0002);
    const NAME: &'static str = "frugalos.config.server.put";

    type Req = Server;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Server>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// サーバ削除RPC。
#[derive(Debug)]
pub struct DeleteServerRpc;
impl Call for DeleteServerRpc {
    const ID: ProcedureId = ProcedureId(0x0002_0003);
    const NAME: &'static str = "frugalos.config.server.delete";

    type Req = ServerId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Server>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// デバイス一覧取得RPC。
#[derive(Debug)]
pub struct ListDevicesRpc;
impl Call for ListDevicesRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0000);
    const NAME: &'static str = "frugalos.config.device.list";

    type Req = ();
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<DeviceSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// デバイス情報取得RPC。
#[derive(Debug)]
pub struct GetDeviceRpc;
impl Call for GetDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0001);
    const NAME: &'static str = "frugalos.config.device.get";

    type Req = DeviceId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Device>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// デバイス登録RPC。
#[derive(Debug)]
pub struct PutDeviceRpc;
impl Call for PutDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0002);
    const NAME: &'static str = "frugalos.config.device.put";

    type Req = Device;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Device>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// デバイス削除RPC。
#[derive(Debug)]
pub struct DeleteDeviceRpc;
impl Call for DeleteDeviceRpc {
    const ID: ProcedureId = ProcedureId(0x0003_0003);
    const NAME: &'static str = "frugalos.config.device.delete";

    type Req = DeviceId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Device>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バケツ一覧取得RPC。
#[derive(Debug)]
pub struct ListBucketsRpc;
impl Call for ListBucketsRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0000);
    const NAME: &'static str = "frugalos.config.bucket.list";

    type Req = ();
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Vec<BucketSummary>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バケツ情報取得RPC。
#[derive(Debug)]
pub struct GetBucketRpc;
impl Call for GetBucketRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0001);
    const NAME: &'static str = "frugalos.config.bucket.get";

    type Req = BucketId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Bucket>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バケツ登録RPC。
#[derive(Debug)]
pub struct PutBucketRpc;
impl Call for PutBucketRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0002);
    const NAME: &'static str = "frugalos.config.bucket.put";

    type Req = Bucket;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Bucket>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// バケツ削除RPC。
#[derive(Debug)]
pub struct DeleteBucketRpc;
impl Call for DeleteBucketRpc {
    const ID: ProcedureId = ProcedureId(0x0004_0003);
    const NAME: &'static str = "frugalos.config.bucket.delete";

    type Req = BucketId;
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<Option<Bucket>>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}

/// Raftのリーダノード取得RPC。
// NOTE: リーダ選出中の場合にはserver側でwaitする
#[derive(Debug)]
pub struct GetLeaderRpc;
impl Call for GetLeaderRpc {
    const ID: ProcedureId = ProcedureId(0x0005_0000);
    const NAME: &'static str = "frugalos.config.leader.get";

    type Req = ();
    type ReqDecoder = BincodeDecoder<Self::Req>;
    type ReqEncoder = BincodeEncoder<Self::Req>;

    type Res = Result<SocketAddr>;
    type ResDecoder = BincodeDecoder<Self::Res>;
    type ResEncoder = BincodeEncoder<Self::Res>;
}
