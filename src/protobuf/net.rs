//! `package libfrugalos.protobuf.net;`

use bytecodec::ErrorKind;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Oneof};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder};
use std::net::SocketAddr;
use trackable::error::ErrorKindExt;

/// Decoder for [std.net.SocketAddr].
#[derive(Debug, Default)]
pub struct SocketAddrDecoder {
    inner: MessageDecoder<
        Oneof<(
            FieldDecoder<F1, StringDecoder>,
            FieldDecoder<F2, StringDecoder>,
        )>,
    >,
}
impl_message_decode!(SocketAddrDecoder, SocketAddr, |t: Branch2<
    String,
    String,
>| {
    match t {
        Branch2::A(addr) => track!(addr
            .parse()
            .map_err(|e| ErrorKind::InvalidInput.cause(e).into())),
        Branch2::B(addr) => track!(addr
            .parse()
            .map_err(|e| ErrorKind::InvalidInput.cause(e).into())),
    }
});

/// Encoder for [std.net.SocketAddr].
#[derive(Debug, Default)]
pub struct SocketAddrEncoder {
    inner: MessageEncoder<
        Oneof<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, StringEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(SocketAddrEncoder, SocketAddr, |item: Self::Item| {
    match item {
        SocketAddr::V4(addr) => Branch2::A(addr.to_string()),
        SocketAddr::V6(addr) => Branch2::B(addr.to_string()),
    }
});
