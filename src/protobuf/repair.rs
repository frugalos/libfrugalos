//! Decoders and encoders for [`libfrugalos::repair`](../../repair/index.html).
//!
//! `package libfrugalos.protobuf.repair;`

use bytecodec::ErrorKind;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2, F3};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MessageFieldDecoder, MessageFieldEncoder, Oneof, Optional,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::wellknown::google::protobuf::{
    DurationMessage, DurationMessageDecoder, DurationMessageEncoder, EmptyMessageDecoder,
    EmptyMessageEncoder,
};

use protobuf_codec::scalar::{Uint64Decoder, Uint64Encoder};
use repair::{RepairConcurrencyLimit, RepairConfig, RepairIdleness, SegmentGcConcurrencyLimit};

/// Decoder for [`RepairConfig`](../../repair/struct.RepairConfig.html).
#[derive(Debug, Default)]
pub struct RepairConfigDecoder {
    inner: MessageDecoder<
        Fields<(
            Optional<FieldDecoder<F1, Uint64Decoder>>,
            Optional<MessageFieldDecoder<F2, RepairIdlenessDecoder>>,
            Optional<FieldDecoder<F3, Uint64Decoder>>,
        )>,
    >,
}
impl_message_decode!(RepairConfigDecoder, RepairConfig, |t: (
    Option<_>,
    _,
    Option<_>
)| {
    Ok(RepairConfig {
        repair_concurrency_limit: t.0.map(RepairConcurrencyLimit),
        repair_idleness_threshold: t.1,
        segment_gc_concurrency_limit: t.2.map(SegmentGcConcurrencyLimit),
    })
});

/// Encoder for [`RepairConfig`](../../repair/struct.RepairConfig.html).
#[derive(Debug, Default)]
pub struct RepairConfigEncoder {
    inner: MessageEncoder<
        Fields<(
            Optional<FieldEncoder<F1, Uint64Encoder>>,
            Optional<MessageFieldEncoder<F2, RepairIdlenessEncoder>>,
            Optional<FieldEncoder<F3, Uint64Encoder>>,
        )>,
    >,
}
impl_sized_message_encode!(RepairConfigEncoder, RepairConfig, |item: Self::Item| {
    (
        item.repair_concurrency_limit.map(|limit| limit.0),
        item.repair_idleness_threshold,
        item.segment_gc_concurrency_limit.map(|limit| limit.0),
    )
});

/// Decoder for [`RepairIdleness`](../../repair/enum.RepairIdleness.html).
#[derive(Debug, Default)]
pub struct RepairIdlenessDecoder {
    inner: MessageDecoder<
        Oneof<(
            // Threshold
            MessageFieldDecoder<F1, DurationMessageDecoder>,
            // Disabled
            MessageFieldDecoder<F2, EmptyMessageDecoder>,
        )>,
    >,
}
impl_message_decode!(RepairIdlenessDecoder, RepairIdleness, |t: Branch2<
    DurationMessage,
    (),
>| Ok(match t {
    Branch2::A(threshold) => {
        if let Some(duration) = threshold.to_duration() {
            RepairIdleness::Threshold(duration)
        } else {
            track_panic!(
                ErrorKind::InvalidInput,
                "Invalid threshold: {:?}",
                threshold
            );
        }
    }
    Branch2::B(_) => RepairIdleness::Disabled,
}));

/// Encoder for [`RepairIdleness`](../../repair/enum.RepairIdleness.html).
#[derive(Debug, Default)]
pub struct RepairIdlenessEncoder {
    inner: MessageEncoder<
        Oneof<(
            // Threshold
            MessageFieldEncoder<F1, DurationMessageEncoder>,
            // Disabled
            MessageFieldEncoder<F2, EmptyMessageEncoder>,
        )>,
    >,
}
impl ::bytecodec::Encode for RepairIdlenessEncoder {
    type Item = RepairIdleness;

    fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
        let item = match item {
            RepairIdleness::Threshold(duration) => {
                Branch2::A(track!(DurationMessage::from_duration(duration))?)
            }
            RepairIdleness::Disabled => Branch2::B(()),
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
impl ::protobuf_codec::message::MessageEncode for RepairIdlenessEncoder {}
impl ::bytecodec::SizedEncode for RepairIdlenessEncoder {
    fn exact_requiring_bytes(&self) -> u64 {
        self.inner.exact_requiring_bytes()
    }
}
