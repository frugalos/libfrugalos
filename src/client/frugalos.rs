//! Frugalosの公開API用のRPCクライアント。
use fibers_rpc::client::ClientServiceHandle as RpcServiceHandle;
use fibers_rpc::Call as RpcCall;
use futures::Future;
use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::ops::Range;
use std::time::Duration;

use super::Response;
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
        consistency: ReadConsistency,
    ) -> impl Future<Item = Option<(ObjectVersion, Vec<u8>)>, Error = Error> {
        let request = frugalos::ObjectRequest {
            bucket_id,
            object_id,
            deadline,
            expect,
            consistency: Some(consistency),
        };
        Response(frugalos::GetObjectRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `ListObjectsRpc`を実行する。
    pub fn list_objects(
        &self,
        bucket_id: BucketId,
        segment: u16,
        consistency: ReadConsistency,
    ) -> impl Future<Item = Vec<ObjectSummary>, Error = Error> {
        let request = frugalos::ListObjectsRequest {
            bucket_id,
            segment,
            consistency,
        };
        Response(frugalos::ListObjectsRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `ListObjectsByPrefixRpc`を実行する。
    pub fn list_objects_by_prefix(
        &self,
        bucket_id: BucketId,
        prefix: ObjectPrefix,
        deadline: Duration,
    ) -> impl Future<Item = Vec<ObjectSummary>, Error = Error> {
        let request = frugalos::PrefixRequest {
            bucket_id,
            prefix,
            deadline,
        };
        Response(
            frugalos::ListObjectsByPrefixRpc::client(&self.rpc_service).call(self.server, request),
        )
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

    /// `CountFragmentsRpc`を実行する。
    pub fn count_fragments(
        &self,
        bucket_id: BucketId,
        object_id: ObjectId,
        deadline: Duration,
        expect: Expect,
        consistency: ReadConsistency,
    ) -> impl Future<Item = Option<FragmentsSummary>, Error = Error> {
        let request = frugalos::CountFragmentsRequest {
            bucket_id,
            object_id,
            deadline,
            expect,
            consistency,
        };
        Response(frugalos::CountFragmentsRpc::client(&self.rpc_service).call(self.server, request))
    }

    /// `HeadObjectRpc`を実行する。
    pub fn head_object(
        &self,
        bucket_id: BucketId,
        object_id: ObjectId,
        deadline: Duration,
        expect: Expect,
        consistency: ReadConsistency,
        check_storage: bool,
    ) -> impl Future<Item = Option<ObjectVersion>, Error = Error> {
        let request = frugalos::HeadObjectRequest {
            bucket_id,
            object_id,
            deadline,
            expect,
            consistency,
            check_storage,
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
        multiplicity_config: MultiplicityConfig,
    ) -> impl Future<Item = (ObjectVersion, bool), Error = Error> {
        let request = frugalos::PutObjectRequest {
            bucket_id,
            object_id,
            content,
            deadline,
            expect,
            multiplicity_config,
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
            consistency: None,
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

    /// Executes `DeleteObjectSetFromDeviceRpc`.
    pub fn delete_from_device_by_object_ids(
        &self,
        bucket_id: BucketId,
        device_id: DeviceId,
        object_ids: BTreeSet<ObjectId>,
    ) -> impl Future<Item = (), Error = Error> {
        Response(
            frugalos::DeleteObjectSetFromDeviceRpc::client(&self.rpc_service).call(
                self.server,
                frugalos::DeleteObjectSetFromDeviceRequest {
                    bucket_id,
                    device_id,
                    object_ids,
                },
            ),
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

    /// Executes `SetRepairConfigRpc`
    pub fn set_repair_config(
        &self,
        repair_config: RepairConfig,
    ) -> impl Future<Item = (), Error = Error> {
        Response(
            frugalos::SetRepairConfigRpc::client(&self.rpc_service)
                .call(self.server, repair_config),
        )
    }
}
