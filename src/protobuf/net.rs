//! Decoders and encoders for [`std::net`](https://doc.rust-lang.org/stable/std/net/index.html).
//!
//! `package libfrugalos.protobuf.net`.

use bytecodec::ErrorKind;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Oneof};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder};
use std::net::SocketAddr;
use trackable::error::ErrorKindExt;

/// Decoder for [std::net::SocketAddr](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html).
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

/// Encoder for [std::net::SocketAddr](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html).
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

#[cfg(test)]
mod tests {
    use super::*;
    use bytecodec::io::{IoDecodeExt, IoEncodeExt};
    use bytecodec::EncodeExt;
    use trackable::result::TestResult;

    #[test]
    fn encode_v4_works() -> TestResult {
        let mut buf = Vec::new();
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut decoder = SocketAddrDecoder::default();
        let mut encoder = track!(SocketAddrEncoder::with_item(addr.clone()))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(
            buf,
            [10, 14, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 48]
        );
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(addr, message);
        Ok(())
    }

    #[test]
    fn encode_v6_works() -> TestResult {
        let mut buf = Vec::new();
        let addr: SocketAddr = "[::1]:12345".parse().unwrap();
        let mut decoder = SocketAddrDecoder::default();
        let mut encoder = track!(SocketAddrEncoder::with_item(addr.clone()))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [18, 11, 91, 58, 58, 49, 93, 58, 49, 50, 51, 52, 53]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(addr, message);
        Ok(())
    }
}
