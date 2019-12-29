//! [frugalos]の公開インタフェースおよびRPCクライアントを提供するためのクレート。
//!
//! [frugalos]: https://github.com/frugalos/frugalos
#![warn(missing_docs)]
extern crate bytecodec;
extern crate fibers;
extern crate fibers_rpc;
extern crate futures;
extern crate libc;
extern crate protobuf_codec;
extern crate serde;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};

pub mod client;
pub mod consistency;
pub mod deadline;
pub mod entity;
pub mod expect;
#[macro_use]
mod macros;
pub mod multiplicity;
pub mod protobuf;
pub mod repair;
pub mod schema;
pub mod time;

mod error;

/// クレート固有の`Result`型。
pub type Result<T> = std::result::Result<T, Error>;
