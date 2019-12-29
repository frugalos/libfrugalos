//! Decoders and encoders for [libfrugalos.entity].

use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Fields, MaybeDefault};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder};
use std::net::SocketAddr;
use std::str::FromStr;

use entity::node::RemoteNodeId;

/// Decoder for [LocalNodeId].
pub type LocalNodeIdDecoder = StringDecoder;

/// Encoder for [LocalNodeId].
pub type LocalNodeIdEncoder = StringEncoder;

/// Decoder for `RemoteNodeId`.
#[derive(Debug, Default)]
pub struct RemoteNodeIdDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
        )>,
    >,
}

impl_message_decode!(RemoteNodeIdDecoder, RemoteNodeId, |t: (String, String,)| {
    let addr = track_any_err!(SocketAddr::from_str(&t.0))?;
    Ok((addr, t.1))
});

/// Encoder for `RemoteNodeId`.
#[derive(Debug, Default)]
pub struct RemoteNodeIdEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, StringEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(RemoteNodeIdEncoder, RemoteNodeId, |item: Self::Item| {
    (item.0.to_string(), item.1)
});
