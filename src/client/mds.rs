//! MDS(metadata store)用のRPCクライアント。
use fibers_rpc::client::ClientServiceHandle as RpcServiceHandle;
use fibers_rpc::{Call as RpcCall, Cast as RpcCast};
use futures::{Async, Future, Poll};
use std::ops::Range;
use std::time::Duration;

use super::Response;
use entity::node::{LocalNodeId, RemoteNodeId};
use entity::object::{
    DeleteObjectsByPrefixSummary, Metadata, ObjectId, ObjectPrefix, ObjectSummary, ObjectVersion,
};
use expect::Expect;
use schema::mds;
use {Error, ErrorKind, Result};

/// RPCクライアント。
#[derive(Debug)]
pub struct Client {
    node: RemoteNodeId,
    rpc_service: RpcServiceHandle,
}
impl Client {
    /// 新しい`Client`インスタンスを生成する。
    pub fn new(node: RemoteNodeId, rpc_service: RpcServiceHandle) -> Self {
        Client { node, rpc_service }
    }

    /// `RecommendToLeaderRpc`を実行する。
    pub fn recommend_to_leader(&self) {
        let _ = mds::RecommendToLeaderRpc::client(&self.rpc_service)
            .cast(self.node.0, self.node.1.clone());
    }

    /// `ListObjectsRpc`を実行する。
    pub fn list_objects(
        &self,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Vec<ObjectSummary>), Error = Error> {
        Call::<mds::ListObjectsRpc, _>::new(self, self.node.1.clone())
    }

    /// `GetLatestVersionRpc`を実行する。
    pub fn latest_version(
        &self,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Option<ObjectSummary>), Error = Error> {
        Call::<mds::GetLatestVersionRpc, _>::new(self, self.node.1.clone())
    }

    /// セグメントが保持しているオブジェクトの数を返す.
    pub fn object_count(&self) -> impl Future<Item = (Option<RemoteNodeId>, u64), Error = Error> {
        Call::<mds::GetObjectCountRpc, _>::new(self, self.node.1.clone())
    }

    /// `GetObjectRpc`を実行する。
    pub fn get_object(
        &self,
        id: ObjectId,
        expect: Expect,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Option<Metadata>), Error = Error> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
        };
        Call::<mds::GetObjectRpc, _>::new(self, request)
    }

    /// `MdsHeadObjectRpc`を実行する。
    pub fn mds_head_object(
        &self,
        id: ObjectId,
        expect: Expect,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Option<ObjectVersion>), Error = Error> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
        };
        Call::<mds::MdsHeadObjectRpc, _>::new(self, request)
    }

    /// `HeadObjectRpc`を実行する。
    pub fn head_object(
        &self,
        id: ObjectId,
        expect: Expect,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Option<ObjectVersion>), Error = Error> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
        };
        Call::<mds::HeadObjectRpc, _>::new(self, request)
    }

    /// `PutObjectRpc`を実行する。
    pub fn put_object(
        &self,
        id: ObjectId,
        metadata: Vec<u8>,
        expect: Expect,
        put_content_timeout: Duration,
    ) -> impl Future<Item = (Option<RemoteNodeId>, (ObjectVersion, Option<ObjectVersion>)), Error = Error>
    {
        let request = mds::PutObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            metadata,
            expect,
            put_content_timeout,
        };
        Call::<mds::PutObjectRpc, _>::new(self, request)
    }

    /// `DeleteObjectRpc`を実行する。
    pub fn delete_object(
        &self,
        id: ObjectId,
        expect: Expect,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Option<ObjectVersion>), Error = Error> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
        };
        Call::<mds::DeleteObjectRpc, _>::new(self, request)
    }

    /// `DeleteObjectByVersionRpc`を実行する。
    pub fn delete_object_by_version(
        &self,
        version: ObjectVersion,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Option<ObjectVersion>), Error = Error> {
        let request = mds::VersionRequest {
            node_id: self.node.1.clone(),
            object_version: version,
        };
        Call::<mds::DeleteObjectByVersionRpc, _>::new(self, request)
    }

    /// `DeleteObjectsByRangeRpc`を実行する。
    pub fn delete_by_range(
        &self,
        targets: Range<ObjectVersion>,
    ) -> impl Future<Item = (Option<RemoteNodeId>, Vec<ObjectSummary>), Error = Error> {
        let request = mds::RangeRequest {
            node_id: self.node.1.clone(),
            targets,
        };
        Call::<mds::DeleteObjectsByRangeRpc, _>::new(self, request)
    }

    /// `DeleteObjectsByPrefixRpc`を実行する。
    pub fn delete_by_prefix(
        &self,
        prefix: ObjectPrefix,
    ) -> impl Future<Item = (Option<RemoteNodeId>, DeleteObjectsByPrefixSummary), Error = Error>
    {
        let request = mds::PrefixRequest {
            node_id: self.node.1.clone(),
            prefix,
        };
        Call::<mds::DeleteObjectsByPrefixRpc, _>::new(self, request)
    }
}

trait SetNodeId {
    fn set_node_id(&mut self, node_id: LocalNodeId);
}
impl SetNodeId for LocalNodeId {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        *self = node_id;
    }
}
impl SetNodeId for mds::ObjectRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}
impl SetNodeId for mds::VersionRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}
impl SetNodeId for mds::RangeRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}
impl SetNodeId for mds::PrefixRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}
impl SetNodeId for mds::PutObjectRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}

#[derive(Debug)]
struct Call<T: RpcCall, U> {
    node: RemoteNodeId,
    rpc_service: RpcServiceHandle,
    leader: Option<Response<RemoteNodeId>>,
    request: T::Req,
    response: Option<Response<U>>,
    retried_count: usize,
}
impl<T: RpcCall, U> Call<T, U>
where
    U: Send + 'static,
    T: RpcCall<Res = Result<U>>,
    T::Req: Clone + SetNodeId,
    T::ReqEncoder: Default,
    T::ResDecoder: Default,
{
    fn new(client: &Client, request: T::Req) -> Self {
        let future = T::client(&client.rpc_service).call(client.node.0, request.clone());
        Call {
            node: client.node.clone(),
            rpc_service: client.rpc_service.clone(),
            leader: None,
            request,
            response: Some(Response(future)),
            retried_count: 0,
        }
    }
}
impl<T, U> Future for Call<T, U>
where
    U: Send + 'static,
    T: RpcCall<Res = Result<U>>,
    T::Req: Clone + SetNodeId,
    T::ReqEncoder: Default,
    T::ResDecoder: Default,
{
    type Item = (Option<RemoteNodeId>, U);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.response.poll() {
                Err(e) => {
                    if *e.kind() == ErrorKind::NotLeader {
                        track_assert!(
                            self.retried_count < 2,
                            ErrorKind::Unavailable,
                            "Unstable cluster: RPC={}",
                            T::NAME
                        );

                        self.retried_count += 1;
                        let future = mds::GetLeaderRpc::client(&self.rpc_service)
                            .call(self.node.0, self.node.1.clone());
                        self.leader = Some(Response(future));
                        self.response = None;
                    } else {
                        return Err(track!(e, T::NAME));
                    }
                }
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(None)) => {}
                Ok(Async::Ready(Some(response))) => {
                    let new_leader = self.leader.as_ref().map(|_| self.node.clone());
                    return Ok(Async::Ready((new_leader, response)));
                }
            }

            if let Async::Ready(Some(leader)) = track!(self.leader.poll())? {
                self.node = leader;
                self.request.set_node_id(self.node.1.clone());
                let future = T::client(&self.rpc_service).call(self.node.0, self.request.clone());
                self.response = Some(Response(future));
            } else {
                break;
            }
        }
        Ok(Async::NotReady)
    }
}
