//! Decoders and encoders for [`libfrugalos::entity::object`](../../entity/object/index.html).
//!
//! `package libfrugalos.protobuf.entity.object`.

use protobuf_codec::field::num::{F1, F2, F3};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, PackedFieldDecoder, PackedFieldEncoder,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    BoolDecoder, BoolEncoder, BytesDecoder, BytesEncoder, StringDecoder, StringEncoder,
    Uint32Decoder, Uint32Encoder, Uint64Decoder, Uint64Encoder,
};
use std::ops::Range;

use entity::object::{
    DeleteObjectsByPrefixSummary, FragmentsSummary, Metadata, ObjectPrefix, ObjectSummary,
    ObjectVersion,
};

/// Decoder for `ObjectId`.
pub type ObjectIdDecoder = StringDecoder;

/// Encoder for `ObjectId`.
pub type ObjectIdEncoder = StringEncoder;

/// Decoder for `ObjectVersion`.
pub type ObjectVersionDecoder = Uint64Decoder;

/// Encoder for `ObjectVersion`.
pub type ObjectVersionEncoder = Uint64Encoder;

/// Decoder for `ObjectVersion`s.
// 互換性に注意
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L185
#[derive(Debug, Default)]
pub struct ObjectVersionsDecoder {
    inner: MessageDecoder<PackedFieldDecoder<F1, Uint64Decoder, Vec<u64>>>,
}
impl_message_decode!(ObjectVersionsDecoder, Vec<u64>, |t: _| { Ok(t) });

/// Encoder for `ObjectVersion`s.
// 互換性に注意
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L191
#[derive(Debug, Default)]
pub struct ObjectVersionsEncoder {
    inner: MessageEncoder<PackedFieldEncoder<F1, Uint64Encoder, Vec<u64>>>,
}
impl_message_encode!(ObjectVersionsEncoder, Vec<u64>, |item: Self::Item| item);

/// Decoder for `ObjectRange`.
#[derive(Debug, Default)]
pub struct ObjectRangeDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, ObjectVersionDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectVersionDecoder>>,
        )>,
    >,
}

impl_message_decode!(ObjectRangeDecoder, Range<ObjectVersion>, |t: (u64, u64)| {
    Ok(Range {
        start: ObjectVersion(t.0),
        end: ObjectVersion(t.1),
    })
});

/// Encoder for `ObjectRange`.
#[derive(Debug, Default)]
pub struct ObjectRangeEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, ObjectVersionEncoder>,
            FieldEncoder<F2, ObjectVersionEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(
    ObjectRangeEncoder,
    Range<ObjectVersion>,
    |item: Self::Item| { (item.start.0, item.end.0) }
);

/// Decoder for `ObjectSummary`.
#[derive(Debug, Default)]
pub struct ObjectSummaryDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, ObjectIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectVersionDecoder>>,
        )>,
    >,
}

impl_message_decode!(ObjectSummaryDecoder, ObjectSummary, |t: (_, u64)| {
    Ok(ObjectSummary {
        id: t.0,
        version: ObjectVersion(t.1),
    })
});

/// Encoder for `ObjectSummary`.
#[derive(Debug, Default)]
pub struct ObjectSummaryEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, ObjectIdEncoder>,
            FieldEncoder<F2, ObjectVersionEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(ObjectSummaryEncoder, ObjectSummary, |item: Self::Item| {
    (item.id, item.version.0)
});

/// Decoder for `ObjectPrefix`.
#[derive(Debug, Default)]
pub struct ObjectPrefixDecoder {
    inner: MessageDecoder<MaybeDefault<FieldDecoder<F1, StringDecoder>>>,
}

impl_message_decode!(ObjectPrefixDecoder, ObjectPrefix, |t: _| {
    Ok(ObjectPrefix(t))
});

/// Encoder for `ObjectPrefix`.
#[derive(Debug, Default)]
pub struct ObjectPrefixEncoder {
    inner: MessageEncoder<FieldEncoder<F1, StringEncoder>>,
}

impl_sized_message_encode!(ObjectPrefixEncoder, ObjectPrefix, |item: Self::Item| {
    item.0
});

/// Decoder for `DeleteObjectsByPrefixSummary`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByPrefixSummaryDecoder {
    inner: MessageDecoder<MaybeDefault<FieldDecoder<F1, Uint64Decoder>>>,
}

impl_message_decode!(
    DeleteObjectsByPrefixSummaryDecoder,
    DeleteObjectsByPrefixSummary,
    |total: _| { Ok(DeleteObjectsByPrefixSummary { total }) }
);

/// Encoder for `DeleteObjectsByPrefixSummary`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByPrefixSummaryEncoder {
    inner: MessageEncoder<FieldEncoder<F1, Uint64Encoder>>,
}

impl_sized_message_encode!(
    DeleteObjectsByPrefixSummaryEncoder,
    DeleteObjectsByPrefixSummary,
    |item: Self::Item| { item.total }
);

/// Decoder for `Metadata`.
#[derive(Debug, Default)]
pub struct MetadataDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, ObjectVersionDecoder>>,
            MaybeDefault<FieldDecoder<F2, BytesDecoder>>,
        )>,
    >,
}

impl_message_decode!(MetadataDecoder, Metadata, |t: (_, _)| {
    Ok(Metadata {
        version: ObjectVersion(t.0),
        data: t.1,
    })
});

/// Encoder for `Metadata`.
#[derive(Debug, Default)]
pub struct MetadataEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, ObjectVersionEncoder>,
            FieldEncoder<F2, BytesEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(MetadataEncoder, Metadata, |item: Self::Item| {
    (item.version.0, item.data)
});

/// Decoder for `FragmentsSummary`.
#[derive(Debug, Default)]
pub struct FragmentsSummaryDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BoolDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(FragmentsSummaryDecoder, FragmentsSummary, |t: (_, _, _)| {
    Ok(FragmentsSummary {
        is_corrupted: t.0,
        found_total: t.1 as u8,
        lost_total: t.2 as u8,
    })
});

/// Encoder for `FragmentsSummary`.
#[derive(Debug, Default)]
pub struct FragmentsSummaryEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BoolEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, Uint32Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(
    FragmentsSummaryEncoder,
    FragmentsSummary,
    |item: Self::Item| {
        (
            item.is_corrupted,
            item.found_total as u32,
            item.lost_total as u32,
        )
    }
);
