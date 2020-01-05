//! Common decoders and encoders for schemas.

use protobuf_codec::field::num::F1;
use protobuf_codec::field::{FieldDecoder, FieldEncoder};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};

use entity::object::ObjectVersion;
use protobuf::entity::object::{
    DeleteObjectsByPrefixSummaryDecoder, DeleteObjectsByPrefixSummaryEncoder, ObjectSummaryDecoder,
    ObjectSummaryEncoder, ObjectVersionDecoder, ObjectVersionEncoder,
};
use protobuf::{
    OptionDecoder, OptionEncoder, ResultDecoder, ResultEncoder, VecDecoder, VecEncoder,
};
use Result;

/// Decoder for a response of `DeleteObjectsByPrefixSummary`.
pub type DeleteObjectsByPrefixSummaryResponseDecoder =
    ResultDecoder<DeleteObjectsByPrefixSummaryDecoder>;

/// Encoder for a response of `DeleteObjectsByPrefixSummary`.
pub type DeleteObjectsByPrefixSummaryResponseEncoder =
    ResultEncoder<DeleteObjectsByPrefixSummaryEncoder>;

/// Decoder for a response of `Option<ObjectSummary>`.
pub type MaybeObjectSummaryResponseDecoder = ResultDecoder<OptionDecoder<ObjectSummaryDecoder>>;

/// Encoder for a response of `Option<ObjectSummary>`.
pub type MaybeObjectSummaryResponseEncoder = ResultEncoder<OptionEncoder<ObjectSummaryEncoder>>;

/// Decoder for a response of `Vec<ObjectSummary>`.
pub type ObjectSummarySequenceResponseDecoder = ResultDecoder<VecDecoder<ObjectSummaryDecoder>>;

/// Encoder for a response of `Vec<ObjectSummary>`.
pub type ObjectSummarySequenceResponseEncoder = ResultEncoder<VecEncoder<ObjectSummaryEncoder>>;

/// Decoder for a response of `Option<ObjectVersion>`.
#[derive(Debug, Default)]
pub struct MaybeObjectVersionResponseDecoder {
    inner: ResultDecoder<OptionDecoder<MessageDecoder<FieldDecoder<F1, ObjectVersionDecoder>>>>,
}
impl_message_decode!(
    MaybeObjectVersionResponseDecoder,
    Result<Option<ObjectVersion>>,
    |t: Result<Option<u64>>| Ok(t.map(|x| x.map(ObjectVersion)))
);

/// Encoder for a response of `Option<ObjectVersion>`.
#[derive(Debug, Default)]
pub struct MaybeObjectVersionResponseEncoder {
    inner: ResultEncoder<OptionEncoder<MessageEncoder<FieldEncoder<F1, ObjectVersionEncoder>>>>,
}
impl_message_encode!(
    MaybeObjectVersionResponseEncoder,
    Result<Option<ObjectVersion>>,
    |item: Self::Item| item.map(|x| x.map(|v| v.0))
);
