//! 構成管理系API用のRPCクライアント。
use fibers_rpc::client::ClientServiceHandle as RpcServiceHandle;
use fibers_rpc::Call as RpcCall;
use futures03::Future;
use pin_project::pin_project;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use super::Response;
use crate::entity::bucket::{Bucket, BucketId, BucketSummary};
use crate::entity::device::{Device, DeviceId, DeviceSummary};
use crate::entity::server::{Server, ServerId, ServerSummary};
use crate::schema::config;
use crate::{ErrorKind, Result};

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
    pub fn list_servers(&self) -> impl Future<Output = Result<Vec<ServerSummary>>> {
        Call::<config::ListServersRpc, _>::new(self, ())
    }

    /// `GetServerRpc`を実行する。
    pub fn get_server(&self, server: ServerId) -> impl Future<Output = Result<Option<Server>>> {
        Call::<config::GetServerRpc, _>::new(self, server)
    }

    /// `PutServerRpc`を実行する。
    pub fn put_server(&self, server: Server) -> impl Future<Output = Result<Server>> {
        Call::<config::PutServerRpc, _>::new(self, server)
    }

    /// `DeleteServerRpc`を実行する。
    pub fn delete_server(&self, server: ServerId) -> impl Future<Output = Result<Option<Server>>> {
        Call::<config::DeleteServerRpc, _>::new(self, server)
    }

    /// `ListDevicesRpc`を実行する。
    pub fn list_devices(&self) -> impl Future<Output = Result<Vec<DeviceSummary>>> {
        Call::<config::ListDevicesRpc, _>::new(self, ())
    }

    /// `GetDeviceRpc`を実行する。
    pub fn get_device(&self, device: DeviceId) -> impl Future<Output = Result<Option<Device>>> {
        Call::<config::GetDeviceRpc, _>::new(self, device)
    }

    /// `PutDeviceRpc`を実行する。
    pub fn put_device(&self, device: Device) -> impl Future<Output = Result<Device>> {
        Call::<config::PutDeviceRpc, _>::new(self, device)
    }

    /// `DeleteDeviceRpc`を実行する。
    pub fn delete_device(&self, device: DeviceId) -> impl Future<Output = Result<Option<Device>>> {
        Call::<config::DeleteDeviceRpc, _>::new(self, device)
    }

    /// `ListBucketsRpc`を実行する。
    pub fn list_buckets(&self) -> impl Future<Output = Result<Vec<BucketSummary>>> {
        Call::<config::ListBucketsRpc, _>::new(self, ())
    }

    /// `GetBucketRpc`を実行する。
    pub fn get_bucket(&self, bucket: BucketId) -> impl Future<Output = Result<Option<Bucket>>> {
        Call::<config::GetBucketRpc, _>::new(self, bucket)
    }

    /// `PutBucketRpc`を実行する。
    pub fn put_bucket(&self, bucket: Bucket) -> impl Future<Output = Result<Bucket>> {
        Call::<config::PutBucketRpc, _>::new(self, bucket)
    }

    /// `DeleteBucketRpc`を実行する。
    pub fn delete_bucket(&self, bucket: BucketId) -> impl Future<Output = Result<Option<Bucket>>> {
        Call::<config::DeleteBucketRpc, _>::new(self, bucket)
    }
}

#[pin_project]
#[derive(Debug)]
struct Call<T: RpcCall, U> {
    contact_server: SocketAddr,
    rpc_service: RpcServiceHandle,
    #[pin]
    leader: Response<SocketAddr>,
    #[pin]
    request: T::Req,
    #[pin]
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
    type Output = Result<U>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            let mut this = self.as_mut().project();
            match this.response.as_mut().as_pin_mut().map(|fut| fut.poll(cx)) {
                None => {}
                Some(result) => match result {
                    Poll::Ready(Err(e)) => {
                        if *e.kind() == ErrorKind::NotLeader {
                            if let Err(e) = (|| {
                                track_assert!(
                                    !*this.is_retried,
                                    ErrorKind::Unavailable,
                                    "Unstable cluster: RPC={}",
                                    T::NAME
                                );
                                Ok(())
                            })() {
                                return Poll::Ready(Err(e));
                            }

                            *this.is_retried = true;
                            let future = config::GetLeaderRpc::client(&this.rpc_service)
                                .call(*this.contact_server, ());
                            this.leader.set(Response(future));
                            this.response.set(None);
                        } else {
                            return Poll::Ready(Err(track!(e, T::NAME)));
                        }
                    }
                    Poll::Pending => break,
                    Poll::Ready(Ok(response)) => return Poll::Ready(Ok(response)),
                },
            }

            if let Poll::Ready(leader) = track!(this.leader.poll(cx))? {
                let future = T::client(&this.rpc_service).call(leader, this.request.clone());
                this.response.set(Some(Response(future)));
            } else {
                break;
            }
        }
        Poll::Pending
    }
}
