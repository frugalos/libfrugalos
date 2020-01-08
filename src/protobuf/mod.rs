//! Encoders and decoders of Protocol Buffers.
#![allow(clippy::type_complexity)]

use bytecodec::combinator::PreEncode;
use bytecodec::SizedEncode;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{MessageFieldDecoder, MessageFieldEncoder, Oneof, Optional, Repeated};
use protobuf_codec::message::{MessageDecode, MessageDecoder, MessageEncode, MessageEncoder};

use protobuf::error::{ErrorDecoder, ErrorEncoder};
use Result;

pub mod consistency;
pub mod deadline;
pub mod entity;
pub mod error;
pub mod expect;
pub mod multiplicity;
pub mod net;
pub mod repair;
pub mod schema;

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
            Branch2::A(value) => Ok(track!(Ok(value))),
            Branch2::B(e) => Ok(track!(Err(e.into()))),
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
            Err(e) => Branch2::B(e.into()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use bytecodec::io::{IoDecodeExt, IoEncodeExt};
    use bytecodec::EncodeExt;
    use protobuf_codec::field::{FieldDecoder, FieldEncoder};
    use protobuf_codec::scalar::{Uint64Decoder, Uint64Encoder};
    use trackable::result::TestResult;

    use {Error, ErrorKind};

    #[test]
    fn encode_empty_vec_works() -> TestResult {
        type Decoder = VecDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>;
        type Encoder = VecEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>;
        let mut buf = Vec::new();
        let mut decoder = Decoder::default();
        let mut encoder = track!(Encoder::with_item(vec![]))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, []);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        let expected: Vec<u64> = vec![];
        assert_eq!(expected, message);
        Ok(())
    }

    #[test]
    fn encode_vec_works() -> TestResult {
        type Decoder = VecDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>;
        type Encoder = VecEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>;
        let mut buf = Vec::new();
        let mut decoder = Decoder::default();
        let mut encoder = track!(Encoder::with_item(vec![3, 9, 8]))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [10, 2, 8, 3, 10, 2, 8, 9, 10, 2, 8, 8]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(vec![3, 9, 8], message);
        Ok(())
    }

    #[test]
    fn encode_some_works() -> TestResult {
        type Decoder = OptionDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>;
        type Encoder = OptionEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>;
        let mut buf = Vec::new();
        let mut decoder = Decoder::default();
        let mut encoder = track!(Encoder::with_item(Some(9)))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [10, 2, 8, 9]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(Some(9), message);
        Ok(())
    }

    #[test]
    fn encode_none_works() -> TestResult {
        type Decoder = OptionDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>;
        type Encoder = OptionEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>;
        let mut buf = Vec::new();
        let mut decoder = Decoder::default();
        let mut encoder = track!(Encoder::with_item(None))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, []);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(None, message);
        Ok(())
    }

    #[test]
    fn encode_ok_works() -> TestResult {
        type Decoder = ResultDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>;
        type Encoder = ResultEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>;
        let mut buf = Vec::new();
        let mut decoder = Decoder::default();
        let mut encoder = track!(Encoder::with_item(Ok(7)))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [10, 2, 8, 7]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(7, message.unwrap_or(0));
        Ok(())
    }

    #[test]
    fn encode_err_works() -> TestResult {
        type Decoder = ResultDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>;
        type Encoder = ResultEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>;
        let mut buf = Vec::new();
        let mut decoder = Decoder::default();
        let mut encoder = track!(Encoder::with_item(Err(ErrorKind::Timeout.into())))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [18, 4, 10, 2, 26, 0]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        if let Err(e) = message {
            assert_eq!(ErrorKind::Timeout, *e.kind());
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other).into())
        }
    }
}
