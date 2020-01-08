//! Decoders and encoders for [`libfrugalos::consistency`](../../consistency/index.html).
//!
//! `package libfrugalos.protobuf.consistency`.

use protobuf_codec::field::branch::Branch4;
use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, MessageFieldDecoder, MessageFieldEncoder, Oneof,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{Uint32Decoder, Uint32Encoder};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};

use consistency::ReadConsistency;

/// Decoder for [`ReadConsistency`](../../consistency/enum.ReadConsistency.html).
#[derive(Debug, Default)]
pub struct ReadConsistencyDecoder {
    inner: MessageDecoder<
        Oneof<(
            // Consistent
            MessageFieldDecoder<F1, EmptyMessageDecoder>,
            // Stale
            MessageFieldDecoder<F2, EmptyMessageDecoder>,
            // Quorum
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

/// Encoder for [`ReadConsistency`](../../consistency/enum.ReadConsistency.html).
#[derive(Debug, Default)]
pub struct ReadConsistencyEncoder {
    inner: MessageEncoder<
        Oneof<(
            // Consistent
            MessageFieldEncoder<F1, EmptyMessageEncoder>,
            // Stale
            MessageFieldEncoder<F2, EmptyMessageEncoder>,
            // Quorum
            MessageFieldEncoder<F3, EmptyMessageEncoder>,
            // Subset
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

#[cfg(test)]
mod tests {
    use super::*;
    use bytecodec::io::{IoDecodeExt, IoEncodeExt};
    use bytecodec::EncodeExt;
    use trackable::result::TestResult;

    #[test]
    fn encode_consistent_works() -> TestResult {
        let mut buf = Vec::new();
        let mut decoder = ReadConsistencyDecoder::default();
        let mut encoder = track!(ReadConsistencyEncoder::with_item(
            ReadConsistency::Consistent
        ))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [10, 0]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(ReadConsistency::Consistent, message);
        Ok(())
    }

    #[test]
    fn encode_stale_works() -> TestResult {
        let mut buf = Vec::new();
        let mut decoder = ReadConsistencyDecoder::default();
        let mut encoder = track!(ReadConsistencyEncoder::with_item(ReadConsistency::Stale))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [18, 0]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(ReadConsistency::Stale, message);
        Ok(())
    }

    #[test]
    fn encode_quorum_works() -> TestResult {
        let mut buf = Vec::new();
        let mut decoder = ReadConsistencyDecoder::default();
        let mut encoder = track!(ReadConsistencyEncoder::with_item(ReadConsistency::Quorum))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [26, 0]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(ReadConsistency::Quorum, message);
        Ok(())
    }

    #[test]
    fn encode_subset_works() -> TestResult {
        let mut buf = Vec::new();
        let mut decoder = ReadConsistencyDecoder::default();
        let mut encoder = track!(ReadConsistencyEncoder::with_item(ReadConsistency::Subset(
            5
        )))?;
        track!(encoder.inner.encode_all(&mut buf))?;
        assert_eq!(buf, [32, 5]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(ReadConsistency::Subset(5), message);
        Ok(())
    }
}
