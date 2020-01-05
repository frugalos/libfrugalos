//! Encoders and decoders of Protocol Buffers.
#![allow(clippy::type_complexity)]

use bytecodec::combinator::PreEncode;
use bytecodec::SizedEncode;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, MessageFieldDecoder, MessageFieldEncoder, Oneof, Optional, Repeated,
};
use protobuf_codec::message::{MessageDecode, MessageDecoder, MessageEncode, MessageEncoder};
use protobuf_codec::scalar::{Uint64Decoder, Uint64Encoder};
use trackable::error::ErrorKindExt;

use protobuf::error::{ErrorDecoder, ErrorEncoder};
use {ErrorKind, Result};

pub mod consistency;
pub mod deadline;
pub mod entity;
pub mod error;
pub mod expect;
pub mod net;
pub mod repair;
pub mod schema;

/// Decoder for `u64`.
pub type Uint64NewTypeDecoder = MessageDecoder<FieldDecoder<F1, Uint64Decoder>>;
/// Encoder for `u64`.
pub type Uint64NewTypeEncoder = MessageEncoder<FieldEncoder<F1, Uint64Encoder>>;

/// Decoder for [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html).
///
/// This decoder decodes the following message:
///
/// ```protobuf
/// message Vec {
///     repeated T values = 1;
/// }
/// ```
// TODO Vec を汎用化する
#[derive(Debug, Default)]
pub struct VecDecoder<D>
where
    D: MessageDecode,
{
    inner: MessageDecoder<Repeated<MessageFieldDecoder<F1, D>, Vec<D::Item>>>,
}
impl<D: MessageDecode> ::bytecodec::Decode for VecDecoder<D> {
    type Item = Vec<D::Item>;

    fn decode(&mut self, buf: &[u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
        track!(self.inner.finish_decoding())
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<D: MessageDecode> ::protobuf_codec::message::MessageDecode for VecDecoder<D> {}

/// Encoder for [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html).
///
/// This encoder encodes the following message:
///
/// ```protobuf
/// message Vec {
///     repeated T values = 1;
/// }
/// ```
#[derive(Debug, Default)]
pub struct VecEncoder<E>
where
    E: MessageEncode,
{
    inner: MessageEncoder<Repeated<MessageFieldEncoder<F1, PreEncode<E>>, Vec<E::Item>>>,
}
impl<E: MessageEncode> ::bytecodec::Encode for VecEncoder<E> {
    type Item = Vec<E::Item>;

    fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
        track!(self.inner.start_encoding(item))
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<E: MessageEncode> ::protobuf_codec::message::MessageEncode for VecEncoder<E> {}

/// Decoder for [`Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html).
///
/// This decoder decodes the following message:
///
/// ```protobuf
/// message Option {
///     T some = 1;
/// }
/// ```
#[derive(Debug, Default)]
pub struct OptionDecoder<D>
where
    D: MessageDecode,
{
    inner: MessageDecoder<Optional<MessageFieldDecoder<F1, D>>>,
}
impl<D: MessageDecode> ::bytecodec::Decode for OptionDecoder<D> {
    type Item = Option<D::Item>;

    fn decode(&mut self, buf: &[u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
        track!(self.inner.finish_decoding())
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<D: MessageDecode> ::protobuf_codec::message::MessageDecode for OptionDecoder<D> {}

/// Encoder for [`Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html).
///
/// This encoder encodes the following message:
///
/// ```protobuf
/// message Option {
///     T some = 1;
/// }
/// ```
#[derive(Debug, Default)]
pub struct OptionEncoder<E>
where
    E: MessageEncode,
{
    inner: MessageEncoder<Optional<MessageFieldEncoder<F1, PreEncode<E>>>>,
}
impl<E: MessageEncode> ::bytecodec::Encode for OptionEncoder<E> {
    type Item = Option<E::Item>;

    fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
        track!(self.inner.start_encoding(item))
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<E: MessageEncode> ::protobuf_codec::message::MessageEncode for OptionEncoder<E> {}
impl<E: MessageEncode + SizedEncode> ::bytecodec::SizedEncode for OptionEncoder<E> {
    fn exact_requiring_bytes(&self) -> u64 {
        self.inner.exact_requiring_bytes()
    }
}

/// Decoder for [`Result`](https://doc.rust-lang.org/stable/std/result/enum.Result.html).
///
/// This decoder decodes the following message:
///
/// ```protobuf
/// import "libfrugalos/protobuf/error.proto";
///
/// message Result {
///     T ok = 1;
///     Error err = 2;
/// }
/// ```
///
/// # Examples
///
/// ```rust,no_run
/// use libfrugalos::protobuf::ResultDecoder;
/// use libfrugalos::protobuf::entity::object::ObjectVersionDecoder;
/// type PutObjectResponseDecoder = ResultDecoder<ObjectVersionDecoder>;
/// ```
#[derive(Debug, Default)]
pub struct ResultDecoder<D>
where
    D: MessageDecode,
{
    inner: MessageDecoder<
        Oneof<(
            MessageFieldDecoder<F1, D>,
            MessageFieldDecoder<F2, ErrorDecoder>,
        )>,
    >,
}
impl<D: MessageDecode> ::bytecodec::Decode for ResultDecoder<D> {
    type Item = Result<D::Item>;

    fn decode(&mut self, buf: &[u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
        match track!(self.inner.finish_decoding())? {
            Branch2::A(value) => Ok(Ok(value)),
            // TODO InvalidInput 再検討
            Branch2::B(e) => Ok(track!(Err(ErrorKind::InvalidInput.takes_over(e).into()))),
        }
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<D: MessageDecode> ::protobuf_codec::message::MessageDecode for ResultDecoder<D> {}

/// Encoder for [`Result`](https://doc.rust-lang.org/stable/std/result/enum.Result.html).
///
/// This encoder encodes the following message:
///
/// ```protobuf
/// import "libfrugalos/protobuf/error.proto";
///
/// message Result {
///     T ok = 1;
///     Error err = 2;
/// }
/// ```
///
/// # Examples
///
/// ```rust,no_run
/// use libfrugalos::protobuf::ResultEncoder;
/// use libfrugalos::protobuf::entity::object::ObjectVersionEncoder;
/// type PutObjectResponseEncoder = ResultEncoder<ObjectVersionEncoder>;
/// ```
#[derive(Debug, Default)]
pub struct ResultEncoder<E>
where
    E: MessageEncode,
{
    inner: MessageEncoder<
        Oneof<(
            MessageFieldEncoder<F1, PreEncode<E>>,
            MessageFieldEncoder<F2, PreEncode<ErrorEncoder>>,
        )>,
    >,
}
impl<E: MessageEncode> ::bytecodec::Encode for ResultEncoder<E> {
    type Item = Result<E::Item>;

    fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
        let item = match item {
            Ok(x) => Branch2::A(x),
            Err(e) => {
                // TODO InvalidInput 再検討
                Branch2::B(ErrorKind::InvalidInput.takes_over(e))
            }
        };
        track!(self.inner.start_encoding(item))
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<E: MessageEncode> ::protobuf_codec::message::MessageEncode for ResultEncoder<E> {}
impl<E: MessageEncode + SizedEncode> ::bytecodec::SizedEncode for ResultEncoder<E> {
    fn exact_requiring_bytes(&self) -> u64 {
        self.inner.exact_requiring_bytes()
    }
}
