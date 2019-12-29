//! object

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::num::F1;
use protobuf_codec::field::{FieldDecoder, FieldEncoder, MessageFieldDecoder, MessageFieldEncoder};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};

use entity::object::{DeleteObjectsByPrefixSummary, ObjectSummary, ObjectVersion};
use protobuf::entity::object::{
    DeleteObjectsByPrefixSummaryDecoder, DeleteObjectsByPrefixSummaryEncoder, ObjectSummaryDecoder,
    ObjectSummaryEncoder, ObjectVersionDecoder, ObjectVersionEncoder,
};
use protobuf::{
    OptionDecoder, OptionEncoder, ResultDecoder, ResultEncoder, VecDecoder, VecEncoder,
};
use Result;

/// Decoder for a response of `DeleteObjectsByPrefixSummary`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByPrefixSummaryResponseDecoder {
    inner:
        MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<DeleteObjectsByPrefixSummaryDecoder>>>,
}
impl_message_decode!(
    DeleteObjectsByPrefixSummaryResponseDecoder,
    Result<DeleteObjectsByPrefixSummary>,
    |t: _| Ok(t)
);

/// Encoder for a response of `DeleteObjectsByPrefixSummary`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByPrefixSummaryResponseEncoder {
    inner:
        MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<DeleteObjectsByPrefixSummaryEncoder>>>,
}
impl_message_encode!(
    DeleteObjectsByPrefixSummaryResponseEncoder,
    Result<DeleteObjectsByPrefixSummary>,
    |item: Self::Item| item
);

/// Decoder for a response of `Option<ObjectVersion>`.
#[derive(Debug, Default)]
pub struct MaybeObjectVersionResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<OptionDecoder<MessageDecoder<FieldDecoder<F1, ObjectVersionDecoder>>>>,
        >,
    >,
}
impl_message_decode!(
    MaybeObjectVersionResponseDecoder,
    Result<Option<ObjectVersion>>,
    |t: Result<Option<u64>>| Ok(t.map(|x| x.map(ObjectVersion)))
);

/// Encoder for a response of `Option<ObjectVersion>`.
#[derive(Debug, Default)]
pub struct MaybeObjectVersionResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<OptionEncoder<MessageEncoder<FieldEncoder<F1, ObjectVersionEncoder>>>>,
        >,
    >,
}
impl_message_encode!(
    MaybeObjectVersionResponseEncoder,
    Result<Option<ObjectVersion>>,
    |item: Self::Item| item.map(|x| x.map(|v| v.0))
);

/// Decoder for a response of `Option<ObjectSummary>`.
#[derive(Debug, Default)]
pub struct MaybeObjectSummaryResponseDecoder {
    inner:
        MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<ObjectSummaryDecoder>>>>,
}
impl_message_decode!(
    MaybeObjectSummaryResponseDecoder,
    Result<Option<ObjectSummary>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `Option<ObjectSummary>`.
#[derive(Debug, Default)]
pub struct MaybeObjectSummaryResponseEncoder {
    inner:
        MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<ObjectSummaryEncoder>>>>,
}
impl_message_encode!(
    MaybeObjectSummaryResponseEncoder,
    Result<Option<ObjectSummary>>,
    |item: Self::Item| item
);

/// Decoder for a response of `Vec<ObjectSummary>`.
#[derive(Debug, Default)]
pub struct ObjectSummarySequenceResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<ObjectSummaryDecoder>>>>,
}
impl_message_decode!(
    ObjectSummarySequenceResponseDecoder,
    Result<Vec<ObjectSummary>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `Vec<ObjectSummary>`.
#[derive(Debug, Default)]
pub struct ObjectSummarySequenceResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<ObjectSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    ObjectSummarySequenceResponseEncoder,
    Result<Vec<ObjectSummary>>,
    |item: Self::Item| item
);
