//! Decoders and encoders for [libfrugalos.consistency].

use protobuf_codec::field::branch::Branch4;
use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, MessageFieldDecoder, MessageFieldEncoder, Oneof,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{Uint32Decoder, Uint32Encoder};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};

use consistency::ReadConsistency;

/// Decoder for `ReadConsistency`.
#[derive(Debug, Default)]
pub struct ReadConsistencyDecoder {
    inner: MessageDecoder<
        Oneof<(
            MessageFieldDecoder<F1, EmptyMessageDecoder>,
            MessageFieldDecoder<F2, EmptyMessageDecoder>,
            MessageFieldDecoder<F3, EmptyMessageDecoder>,
            FieldDecoder<F4, Uint32Decoder>,
        )>,
    >,
}
impl_message_decode!(ReadConsistencyDecoder, ReadConsistency, |t: _| {
    Ok(match t {
        Branch4::A(_) => ReadConsistency::Consistent,
        Branch4::B(_) => ReadConsistency::Stale,
        Branch4::C(_) => ReadConsistency::Quorum,
        Branch4::D(n) => ReadConsistency::Subset(n as usize),
    })
});

/// Encoder for `ReadConsistency`.
#[derive(Debug, Default)]
pub struct ReadConsistencyEncoder {
    inner: MessageEncoder<
        Oneof<(
            MessageFieldEncoder<F1, EmptyMessageEncoder>,
            MessageFieldEncoder<F2, EmptyMessageEncoder>,
            MessageFieldEncoder<F3, EmptyMessageEncoder>,
            FieldEncoder<F4, Uint32Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ReadConsistencyEncoder,
    ReadConsistency,
    |item: Self::Item| {
        match item {
            ReadConsistency::Consistent => Branch4::A(()),
            ReadConsistency::Stale => Branch4::B(()),
            ReadConsistency::Quorum => Branch4::C(()),
            ReadConsistency::Subset(n) => Branch4::D(n as u32),
        }
    }
);
