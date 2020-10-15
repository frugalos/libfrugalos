//! 構成管理系API用のRPCクライアント。
use fibers_rpc::client::ClientServiceHandle as RpcServiceHandle;
use fibers_rpc::Call as RpcCall;
use futures::{Async, Future, Poll};
use std::net::SocketAddr;

use super::Response;
use crate::entity::bucket::{Bucket, BucketId, BucketSummary};
use crate::entity::device::{Device, DeviceId, DeviceSummary};
use crate::entity::server::{Server, ServerId, ServerSummary};
use crate::schema::config;
use crate::{Error, ErrorKind, Result};

/// RPCクライアント。
#[derive(Debug)]
pub struct Client {
    contact_server: SocketAddr,
    rpc_service: RpcServiceHandle,
}
impl Client {
    /// 新しい`Client`インスタンスを生成する。
    pub fn new(contact_server: SocketAddr, rpc_service: RpcServiceHandle) -> Self {
        Client {
            contact_server,
            rpc_service,
        }
    }

    /// `ListServersRpc`を実行する。
    pub fn list_servers(&self) -> impl Future<Item = Vec<ServerSummary>, Error = Error> {
        Call::<config::ListServersRpc, _>::new(self, ())
    }

    /// `GetServerRpc`を実行する。
    pub fn get_server(
        &self,
        server: ServerId,
    ) -> impl Future<Item = Option<Server>, Error = Error> {
        Call::<config::GetServerRpc, _>::new(self, server)
    }

    /// `PutServerRpc`を実行する。
    pub fn put_server(&self, server: Server) -> impl Future<Item = Server, Error = Error> {
        Call::<config::PutServerRpc, _>::new(self, server)
    }

    /// `DeleteServerRpc`を実行する。
    pub fn delete_server(
        &self,
        server: ServerId,
    ) -> impl Future<Item = Option<Server>, Error = Error> {
        Call::<config::DeleteServerRpc, _>::new(self, server)
    }

    /// `ListDevicesRpc`を実行する。
    pub fn list_devices(&self) -> impl Future<Item = Vec<DeviceSummary>, Error = Error> {
        Call::<config::ListDevicesRpc, _>::new(self, ())
    }

    /// `GetDeviceRpc`を実行する。
    pub fn get_device(
        &self,
        device: DeviceId,
    ) -> impl Future<Item = Option<Device>, Error = Error> {
        Call::<config::GetDeviceRpc, _>::new(self, device)
    }

    /// `PutDeviceRpc`を実行する。
    pub fn put_device(&self, device: Device) -> impl Future<Item = Device, Error = Error> {
        Call::<config::PutDeviceRpc, _>::new(self, device)
    }

    /// `DeleteDeviceRpc`を実行する。
    pub fn delete_device(
        &self,
        device: DeviceId,
    ) -> impl Future<Item = Option<Device>, Error = Error> {
        Call::<config::DeleteDeviceRpc, _>::new(self, device)
    }

    /// `ListBucketsRpc`を実行する。
    pub fn list_buckets(&self) -> impl Future<Item = Vec<BucketSummary>, Error = Error> {
        Call::<config::ListBucketsRpc, _>::new(self, ())
    }

    /// `GetBucketRpc`を実行する。
    pub fn get_bucket(
        &self,
        bucket: BucketId,
    ) -> impl Future<Item = Option<Bucket>, Error = Error> {
        Call::<config::GetBucketRpc, _>::new(self, bucket)
    }

    /// `PutBucketRpc`を実行する。
    pub fn put_bucket(&self, bucket: Bucket) -> impl Future<Item = Bucket, Error = Error> {
        Call::<config::PutBucketRpc, _>::new(self, bucket)
    }

    /// `DeleteBucketRpc`を実行する。
    pub fn delete_bucket(
        &self,
        bucket: BucketId,
    ) -> impl Future<Item = Option<Bucket>, Error = Error> {
        Call::<config::DeleteBucketRpc, _>::new(self, bucket)
    }
}

#[derive(Debug)]
struct Call<T: RpcCall, U> {
    contact_server: SocketAddr,
    rpc_service: RpcServiceHandle,
    leader: Response<SocketAddr>,
    request: T::Req,
    response: Option<Response<U>>,
    is_retried: bool,
}
impl<T, U> Call<T, U>
where
    U: Send + 'static,
    T: RpcCall<Res = Result<U>>,
    T::Req: Clone,
{
    fn new(client: &Client, request: T::Req) -> Self {
        let future =
            config::GetLeaderRpc::client(&client.rpc_service).call(client.contact_server, ());
        Call {
            contact_server: client.contact_server,
            rpc_service: client.rpc_service.clone(),
            leader: Response(future),
            request,
            response: None,
            is_retried: false,
        }
    }
}
impl<T, U> Future for Call<T, U>
where
    U: Send + 'static,
    T: RpcCall<Res = Result<U>>,
    T::Req: Clone,
    T::ReqEncoder: Default,
    T::ResDecoder: Default,
{
    type Item = U;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.response.poll() {
                Err(e) => {
                    if *e.kind() == ErrorKind::NotLeader {
                        track_assert!(
                            !self.is_retried,
                            ErrorKind::Unavailable,
                            "Unstable cluster: RPC={}",
                            T::NAME
                        );

                        self.is_retried = true;
                        let future = config::GetLeaderRpc::client(&self.rpc_service)
                            .call(self.contact_server, ());
                        self.leader = Response(future);
                        self.response = None;
                    } else {
                        return Err(track!(e, T::NAME));
                    }
                }
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(None)) => {}
                Ok(Async::Ready(Some(response))) => return Ok(Async::Ready(response)),
            }

            if let Async::Ready(leader) = track!(self.leader.poll())? {
                let future = T::client(&self.rpc_service).call(leader, self.request.clone());
                self.response = Some(Response(future));
            } else {
                break;
            }
        }
        Ok(Async::NotReady)
    }
}
