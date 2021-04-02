//! RPCクライアント。
use futures03::Future;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use trackable::error::ErrorKindExt;

use crate::{ErrorKind, Result};

pub mod config;
pub mod frugalos;
pub mod mds;

#[pin_project]
#[derive(Debug)]
struct Response<T>(#[pin] fibers_rpc::client::Response<Result<T>>);
impl<T> Future for Response<T> {
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.project().0.poll(cx) {
            Poll::Ready(Err(e)) => {
                let kind = match *e.kind() {
                    fibers_rpc::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
                    fibers_rpc::ErrorKind::Unavailable => ErrorKind::Unavailable,
                    fibers_rpc::ErrorKind::Timeout => ErrorKind::Timeout,
                    fibers_rpc::ErrorKind::Other => ErrorKind::Other,
                };
                Poll::Ready(Err(track!(kind.takes_over(e)).into()))
            }
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(result)) => track!(Poll::Ready(result)),
        }
    }
}
