//! Decoders and encoders for [`libfrugalos::expect`](../../expect/index.html).
//!
//! `package libfrugalos.protobuf.expect`.

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::branch::Branch4;
use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{MessageFieldDecoder, MessageFieldEncoder, Oneof};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};

use entity::object::ObjectVersion;
use expect::Expect;
use protobuf::entity::object::{ObjectVersionsDecoder, ObjectVersionsEncoder};

/// Decoder for [`Expect`](../../expect/enum.Expect.html).
#[derive(Debug, Default)]
pub struct ExpectDecoder {
    inner: MessageDecoder<
        Oneof<(
            // Any
            MessageFieldDecoder<F1, EmptyMessageDecoder>,
            // None
            MessageFieldDecoder<F2, EmptyMessageDecoder>,
            // IfMatch
            MessageFieldDecoder<F3, ObjectVersionsDecoder>,
            // IfNoneMatch
            MessageFieldDecoder<F4, ObjectVersionsDecoder>,
        )>,
    >,
}
impl_message_decode!(ExpectDecoder, Expect, |t: Branch4<
    _,
    _,
    Vec<u64>,
    Vec<u64>,
>| {
    Ok(match t {
        Branch4::A(_) => Expect::Any,
        Branch4::B(_) => Expect::None,
        Branch4::C(versions) => Expect::IfMatch(versions.into_iter().map(ObjectVersion).collect()),
        Branch4::D(versions) => {
            Expect::IfNoneMatch(versions.into_iter().map(ObjectVersion).collect())
        }
    })
});

/// Encoder for [`Expect`](../../expect/enum.Expect.html).
#[derive(Debug, Default)]
pub struct ExpectEncoder {
    inner: MessageEncoder<
        Oneof<(
            // Any
            MessageFieldEncoder<F1, EmptyMessageEncoder>,
            // None
            MessageFieldEncoder<F2, EmptyMessageEncoder>,
            // IfMatch
            MessageFieldEncoder<F3, PreEncode<ObjectVersionsEncoder>>,
            // IfNoneMatch
            MessageFieldEncoder<F4, PreEncode<ObjectVersionsEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(ExpectEncoder, Expect, |item: Self::Item| {
    match item {
        Expect::Any => Branch4::A(()),
        Expect::None => Branch4::B(()),
        Expect::IfMatch(versions) => Branch4::C(versions.into_iter().map(|v| v.0).collect()),
        Expect::IfNoneMatch(versions) => Branch4::D(versions.into_iter().map(|v| v.0).collect()),
    }
});
