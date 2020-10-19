//! RPCクライアント。
use futures::{Async, Future, Poll};
use trackable::error::ErrorKindExt;

use crate::{Error, ErrorKind, Result};

pub mod config;
pub mod frugalos;
pub mod mds;

#[derive(Debug)]
struct Response<T>(fibers_rpc::client::Response<Result<T>>);
impl<T> Future for Response<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0.poll() {
            Err(e) => {
                let kind = match *e.kind() {
                    fibers_rpc::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
                    fibers_rpc::ErrorKind::Unavailable => ErrorKind::Unavailable,
                    fibers_rpc::ErrorKind::Timeout => ErrorKind::Timeout,
                    fibers_rpc::ErrorKind::Other => ErrorKind::Other,
                };
                Err(track!(kind.takes_over(e)).into())
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(result)) => track!(result.map(Async::Ready)),
        }
    }
}
