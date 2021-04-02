//! MDS(metadata store)用のRPCクライアント。
use fibers_rpc::client::ClientServiceHandle as RpcServiceHandle;
use fibers_rpc::{Call as RpcCall, Cast as RpcCast};
use futures03::{future::OptionFuture, Future};
use pin_project::pin_project;
use std::ops::Range;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use super::Response;
use crate::consistency::ReadConsistency;
use crate::entity::node::{LocalNodeId, RemoteNodeId};
use crate::entity::object::{
    DeleteObjectsByPrefixSummary, Metadata, ObjectId, ObjectPrefix, ObjectSummary, ObjectVersion,
};
use crate::expect::Expect;
use crate::schema::mds;
use crate::{ErrorKind, Result};

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
        consistency: ReadConsistency,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Vec<ObjectSummary>)>> {
        let request = mds::ListObjectsRequest {
            node_id: self.node.1.clone(),
            consistency,
        };
        Call::<mds::ListObjectsRpc, _>::new(self, request)
    }

    /// `ListObjectsByPrefixRpc`を実行する。
    pub fn list_objects_by_prefix(
        &self,
        prefix: ObjectPrefix,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Vec<ObjectSummary>)>> {
        let request = mds::PrefixRequest {
            node_id: self.node.1.clone(),
            prefix,
        };
        Call::<mds::ListObjectsByPrefixRpc, _>::new(self, request)
    }

    /// `GetLatestVersionRpc`を実行する。
    pub fn latest_version(
        &self,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Option<ObjectSummary>)>> {
        Call::<mds::GetLatestVersionRpc, _>::new(self, self.node.1.clone())
    }

    /// セグメントが保持しているオブジェクトの数を返す.
    pub fn object_count(
        &self,
        consistency: ReadConsistency,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, u64)>> {
        let request = mds::ObjectCountRequest {
            node_id: self.node.1.clone(),
            consistency,
        };
        Call::<mds::GetObjectCountRpc, _>::new(self, request)
    }

    /// `GetObjectRpc`を実行する。
    pub fn get_object(
        &self,
        id: ObjectId,
        expect: Expect,
        consistency: ReadConsistency,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Option<Metadata>)>> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
            consistency: Some(consistency),
        };
        Call::<mds::GetObjectRpc, _>::new(self, request)
    }

    /// `HeadObjectRpc`を実行する。
    pub fn head_object(
        &self,
        id: ObjectId,
        expect: Expect,
        consistency: ReadConsistency,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Option<ObjectVersion>)>> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
            consistency: Some(consistency),
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
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, (ObjectVersion, Option<ObjectVersion>))>>
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
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Option<ObjectVersion>)>> {
        let request = mds::ObjectRequest {
            node_id: self.node.1.clone(),
            object_id: id,
            expect,
            consistency: None,
        };
        Call::<mds::DeleteObjectRpc, _>::new(self, request)
    }

    /// `DeleteObjectByVersionRpc`を実行する。
    pub fn delete_object_by_version(
        &self,
        version: ObjectVersion,
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Option<ObjectVersion>)>> {
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
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, Vec<ObjectSummary>)>> {
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
    ) -> impl Future<Output = Result<(Option<RemoteNodeId>, DeleteObjectsByPrefixSummary)>> {
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
impl SetNodeId for mds::ListObjectsRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}
impl SetNodeId for mds::ObjectRequest {
    fn set_node_id(&mut self, node_id: LocalNodeId) {
        self.node_id = node_id;
    }
}
impl SetNodeId for mds::ObjectCountRequest {
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

#[pin_project]
#[derive(Debug)]
struct Call<T: RpcCall, U> {
    node: RemoteNodeId,
    rpc_service: RpcServiceHandle,
    #[pin]
    leader: OptionFuture<Response<RemoteNodeId>>,
    is_leader: bool,
    request: T::Req,
    #[pin]
    response: OptionFuture<Response<U>>,
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
            leader: None.into(),
            is_leader: false,
            request,
            response: Some(Response(future)).into(),
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
    type Output = Result<(Option<RemoteNodeId>, U)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            let mut this = self.as_mut().project();
            match this.response.as_mut().poll(cx) {
                Poll::Ready(Some(Err(e))) => {
                    if *e.kind() == ErrorKind::NotLeader {
                        if let Err(e) = (|| {
                            track_assert!(
                                *this.retried_count < 2,
                                ErrorKind::Unavailable,
                                "Unstable cluster: RPC={}",
                                T::NAME
                            );
                            Ok(())
                        })() {
                            return Poll::Ready(Err(e));
                        }

                        *this.retried_count += 1;
                        let future = mds::GetLeaderRpc::client(&this.rpc_service)
                            .call(this.node.0, this.node.1.clone());
                        this.leader.set(Some(Response(future)).into());
                        *this.is_leader = true;
                        this.response.set(None.into());
                    } else {
                        return Poll::Ready(Err(track!(e, T::NAME)));
                    }
                }
                Poll::Pending => break,
                Poll::Ready(None) => {}
                Poll::Ready(Some(Ok(response))) => {
                    let new_leader = if *this.is_leader {
                        Some(this.node.clone())
                    } else {
                        None
                    };
                    return Poll::Ready(Ok((new_leader, response)));
                }
            }

            if let Poll::Ready(Some(leader)) = track!(this.leader.poll(cx))? {
                *this.node = leader;
                this.request.set_node_id(this.node.1.clone());
                let future = T::client(&this.rpc_service).call(this.node.0, this.request.clone());
                this.response.set(Some(Response(future)).into());
            } else {
                break;
            }
        }
        Poll::Pending
    }
}
