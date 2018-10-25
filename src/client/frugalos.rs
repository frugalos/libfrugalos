//! Frugalosの公開API用のRPCクライアント。
use fibers_rpc::client::ClientServiceHandle as RpcServiceHandle;
use fibers_rpc::Call as RpcCall;
use futures::Future;
use std::net::SocketAddr;
use std::ops::Range;
use std::time::Duration;

use super::Response;
use entity::bucket::BucketId;
use entity::object::{
    DeleteObjectsByPrefixSummary, ObjectId, ObjectPrefix, ObjectSummary, ObjectVersion,
};
use expect::Expect;
use schema::frugalos;
use Error;

/// RPCクライアント。
#[derive(Debug)]
pub struct Client {
    server: SocketAddr,
    rpc_service: RpcServiceHandle,
}
impl Client {
    /// 新しい`Client`インスタンスを生成する。
    pub fn new(server: SocketAddr, rpc_service: RpcServiceHandle) -> Self {
        Client {
            server,
            rpc_service,
        }
    }

    /// `GetObjectRpc`を実行する。
    pub fn get_object(
        &self,
        bucket_id: BucketId,
        object_id: ObjectId,
        deadline: Duration,
        expect: Expect,
    ) -> impl Future<Item = Option<(ObjectVersion, Vec<u8>)>, Error = Error> {
        let request = frugalos::ObjectRequest {
            bucket_id,
            object_id,
            deadline,
            expect,
        };
        Response(frugalos::GetObjectRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `ListObjectsRpc`を実行する。
    pub fn list_objects(
        &self,
        bucket_id: BucketId,
        segment: u16,
    ) -> impl Future<Item = Vec<ObjectSummary>, Error = Error> {
        let request = frugalos::SegmentRequest { bucket_id, segment };
        Response(frugalos::ListObjectsRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `GetLatestVersionRpc`を実行する。
    pub fn latest_version(
        &self,
        bucket_id: BucketId,
        segment: u16,
    ) -> impl Future<Item = Option<ObjectSummary>, Error = Error> {
        let request = frugalos::SegmentRequest { bucket_id, segment };
        Response(
            frugalos::GetLatestVersionRpc::client(&self.rpc_service).call(self.server, request),
        )
    }

    /// `HeadObjectRpc`を実行する。
    pub fn head_object(
        &self,
        bucket_id: BucketId,
        object_id: ObjectId,
        deadline: Duration,
        expect: Expect,
    ) -> impl Future<Item = Option<ObjectVersion>, Error = Error> {
        let request = frugalos::ObjectRequest {
            bucket_id,
            object_id,
            deadline,
            expect,
        };
        Response(frugalos::HeadObjectRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `PutObjectRpc`を実行する。
    pub fn put_object(
        &self,
        bucket_id: BucketId,
        object_id: ObjectId,
        content: Vec<u8>,
        deadline: Duration,
        expect: Expect,
    ) -> impl Future<Item = (ObjectVersion, bool), Error = Error> {
        let request = frugalos::PutObjectRequest {
            bucket_id,
            object_id,
            content,
            deadline,
            expect,
        };
        Response(frugalos::PutObjectRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `DeleteObjectRpc`を実行する。
    pub fn delete_object(
        &self,
        bucket_id: BucketId,
        object_id: ObjectId,
        deadline: Duration,
        expect: Expect,
    ) -> impl Future<Item = Option<ObjectVersion>, Error = Error> {
        let request = frugalos::ObjectRequest {
            bucket_id,
            object_id,
            deadline,
            expect,
        };
        Response(frugalos::DeleteObjectRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `DeleteObjectByVersionRpc`を実行する。
    pub fn delete_object_by_version(
        &self,
        bucket_id: BucketId,
        segment: u16,
        object_version: ObjectVersion,
        deadline: Duration,
    ) -> impl Future<Item = Option<ObjectVersion>, Error = Error> {
        let request = frugalos::VersionRequest {
            bucket_id,
            segment,
            object_version,
            deadline,
        };
        Response(
            frugalos::DeleteObjectByVersionRpc::client(&self.rpc_service)
                .call(self.server, request),
        )
    }

    /// `DeleteObjectsByRangeRpc`を実行する。
    pub fn delete_by_range(
        &self,
        bucket_id: BucketId,
        segment: u16,
        targets: Range<ObjectVersion>,
        deadline: Duration,
    ) -> impl Future<Item = Vec<ObjectSummary>, Error = Error> {
        let request = frugalos::RangeRequest {
            bucket_id,
            segment,
            targets,
            deadline,
        };
        Response(
            frugalos::DeleteObjectsByRangeRpc::client(&self.rpc_service).call(self.server, request),
        )
    }

    /// オブジェクトを ID のプレフィックスを指定して削除する。
    pub fn delete_by_prefix(
        &self,
        bucket_id: BucketId,
        prefix: ObjectPrefix,
        deadline: Duration,
    ) -> impl Future<Item = DeleteObjectsByPrefixSummary, Error = Error> {
        let request = frugalos::PrefixRequest {
            bucket_id,
            prefix,
            deadline,
        };
        Response(
            frugalos::DeleteObjectsByPrefixRpc::client(&self.rpc_service)
                .call(self.server, request),
        )
    }

    /// `StopRpc`を実行する。
    pub fn stop(&self) -> impl Future<Item = (), Error = Error> {
        Response(frugalos::StopRpc::client(&self.rpc_service).call(self.server, ()))
    }

    /// `TakeSnapshotRpc`を実行する。
    pub fn take_snapshot(&self) -> impl Future<Item = (), Error = Error> {
        Response(frugalos::TakeSnapshotRpc::client(&self.rpc_service).call(self.server, ()))
    }
}
