//! Decoders and encoders for [`libfrugalos::multiplicity`](../../multiplicity/index.html).
//!
//! `package libfrugalos.protobuf.multiplicity;`

use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Fields, MaybeDefault};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{Uint64Decoder, Uint64Encoder};

use multiplicity::{InnerRetryCount, MultiplicityConfig, NumberOfEnsuredSaves};

/// Decoder for [`MultiplicityConfig`](../../multiplicity/struct.MultiplicityConfig.html).
#[derive(Debug, Default)]
pub struct MultiplicityConfigDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, Uint64Decoder>>,
            MaybeDefault<FieldDecoder<F2, Uint64Decoder>>,
        )>,
    >,
}
impl_message_decode!(MultiplicityConfigDecoder, MultiplicityConfig, |t: (
    _,
    _
)| {
    Ok(MultiplicityConfig {
        inner_retry_count: InnerRetryCount(t.0 as usize),
        number_of_ensured_saves: NumberOfEnsuredSaves(t.1 as usize),
    })
});

/// Encoder for [`MultiplicityConfig`](../../multiplicity/struct.MultiplicityConfig.html).
#[derive(Debug, Default)]
pub struct MultiplicityConfigEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, Uint64Encoder>,
            FieldEncoder<F2, Uint64Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    MultiplicityConfigEncoder,
    MultiplicityConfig,
    |item: Self::Item| {
        (
            item.inner_retry_count.0 as u64,
            item.number_of_ensured_saves.0 as u64,
        )
    }
);
