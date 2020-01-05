//! Decoders and encoders for [`libfrugalos::entity::bucket`](../../entity/bucket/index.html).
//!
//! `package libfrugalos.protobuf.entity.bucket`.

use bytecodec::ErrorKind;
use protobuf_codec::field::branch::Branch3;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5, F6};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder};

use entity::bucket::{
    Bucket, BucketId, BucketKind, BucketSummary, DispersedBucket, MetadataBucket, ReplicatedBucket,
};
use entity::device::DeviceId;
use protobuf::entity::device::{DeviceIdDecoder, DeviceIdEncoder};

/// Decoder for `BucketId`.
pub type BucketIdDecoder = StringDecoder;

/// Encoder for `BucketId`.
pub type BucketIdEncoder = StringEncoder;

/// Decoder for `BucketKind`.
type BucketKindDecoder = Uint32Decoder;

/// Encoder for `BucketKind`.
type BucketKindEncoder = Uint32Encoder;

/// Decoder for `SegmentCount`.
type SegmentCountDecoder = Uint32Decoder;

/// Encoder for `SegmentCount`.
type SegmentCountEncoder = Uint32Encoder;

/// Decoder for `SequenceNumber`.
type SequenceNumberDecoder = Uint32Decoder;

/// Encoder for `SequenceNumber`.
type SequenceNumberEncoder = Uint32Encoder;

/// Decoder for `TolerableFaults`.
type TolerableFaultsDecoder = Uint32Decoder;

/// Encoder for `TolerableFaults`.
type TolerableFaultsEncoder = Uint32Encoder;

/// Decoder for `DataFragmentCount`.
type DataFragmentCountDecoder = Uint32Decoder;

/// Encoder for `DataFragmentCount`.
type DataFragmentCountEncoder = Uint32Encoder;

/// Decoder for `BucketSummary`.
#[derive(Debug, Default)]
pub struct BucketSummaryDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, BucketKindDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeviceIdDecoder>>,
        )>,
    >,
}

impl_message_decode!(BucketSummaryDecoder, BucketSummary, |t: (
    BucketId,
    _,
    DeviceId,
)| {
    Ok(BucketSummary {
        id: t.0,
        kind: match t.1 {
            0 => BucketKind::Metadata,
            1 => BucketKind::Replicated,
            2 => BucketKind::Dispersed,
            n => track_panic!(ErrorKind::InvalidInput, "Unknown bucket kind: {}", n),
        },
        device: t.2,
    })
});

/// Encoder for `BucketSummary`.
#[derive(Debug, Default)]
pub struct BucketSummaryEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, BucketKindEncoder>,
            FieldEncoder<F3, DeviceIdEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(BucketSummaryEncoder, BucketSummary, |item: Self::Item| {
    let kind = match item.kind {
        BucketKind::Metadata => 0,
        BucketKind::Replicated => 1,
        BucketKind::Dispersed => 2,
    };
    (item.id, kind, item.device)
});

/// Decoder for `Bucket`.
#[derive(Debug, Default)]
pub struct BucketDecoder {
    inner: MessageDecoder<
        Oneof<(
            MessageFieldDecoder<F1, MetadataBucketDecoder>,
            MessageFieldDecoder<F2, ReplicatedBucketDecoder>,
            MessageFieldDecoder<F3, DispersedBucketDecoder>,
        )>,
    >,
}

impl_message_decode!(BucketDecoder, Bucket, |t: _| {
    Ok(match t {
        Branch3::A(bucket) => Bucket::Metadata(bucket),
        Branch3::B(bucket) => Bucket::Replicated(bucket),
        Branch3::C(bucket) => Bucket::Dispersed(bucket),
    })
});

/// Encoder for `Bucket`.
#[derive(Debug, Default)]
pub struct BucketEncoder {
    inner: MessageEncoder<
        Oneof<(
            MessageFieldEncoder<F1, MetadataBucketEncoder>,
            MessageFieldEncoder<F2, ReplicatedBucketEncoder>,
            MessageFieldEncoder<F3, DispersedBucketEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(BucketEncoder, Bucket, |item: Self::Item| match item {
    Bucket::Metadata(bucket) => Branch3::A(bucket),
    Bucket::Replicated(bucket) => Branch3::B(bucket),
    Bucket::Dispersed(bucket) => Branch3::C(bucket),
});

/// Decoder for `MetadataBucket`.
#[derive(Debug, Default)]
pub struct MetadataBucketDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, SequenceNumberDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeviceIdDecoder>>,
            MaybeDefault<FieldDecoder<F4, SegmentCountDecoder>>,
            MaybeDefault<FieldDecoder<F5, TolerableFaultsDecoder>>,
        )>,
    >,
}

impl_message_decode!(MetadataBucketDecoder, MetadataBucket, |t: (
    BucketId,
    _,
    DeviceId,
    _,
    _,
)| {
    Ok(MetadataBucket {
        id: t.0.clone(),
        seqno: t.1,
        device: t.2.clone(),
        segment_count: t.3,
        tolerable_faults: t.4,
    })
});

/// Encoder for `MetadataBucket`.
#[derive(Debug, Default)]
pub struct MetadataBucketEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, SequenceNumberEncoder>,
            FieldEncoder<F3, DeviceIdEncoder>,
            FieldEncoder<F4, SegmentCountEncoder>,
            FieldEncoder<F5, TolerableFaultsEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(MetadataBucketEncoder, MetadataBucket, |item: Self::Item| {
    (
        item.id,
        item.seqno,
        item.device,
        item.segment_count,
        item.tolerable_faults,
    )
});

/// Decoder for `ReplicatedBucket`.
#[derive(Debug, Default)]
pub struct ReplicatedBucketDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, SequenceNumberDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeviceIdDecoder>>,
            MaybeDefault<FieldDecoder<F4, SegmentCountDecoder>>,
            MaybeDefault<FieldDecoder<F5, TolerableFaultsDecoder>>,
        )>,
    >,
}

impl_message_decode!(ReplicatedBucketDecoder, ReplicatedBucket, |t: (
    BucketId,
    _,
    DeviceId,
    _,
    _,
)| {
    Ok(ReplicatedBucket {
        id: t.0.clone(),
        seqno: t.1,
        device: t.2.clone(),
        segment_count: t.3,
        tolerable_faults: t.4,
    })
});

/// Encoder for `ReplicatedBucket`.
#[derive(Debug, Default)]
pub struct ReplicatedBucketEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, SequenceNumberEncoder>,
            FieldEncoder<F3, DeviceIdEncoder>,
            FieldEncoder<F4, SegmentCountEncoder>,
            FieldEncoder<F5, TolerableFaultsEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(
    ReplicatedBucketEncoder,
    ReplicatedBucket,
    |item: Self::Item| {
        (
            item.id,
            item.seqno,
            item.device,
            item.segment_count,
            item.tolerable_faults,
        )
    }
);

/// Decoder for `DispersedBucket`.
#[derive(Debug, Default)]
pub struct DispersedBucketDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, SequenceNumberDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeviceIdDecoder>>,
            MaybeDefault<FieldDecoder<F4, SegmentCountDecoder>>,
            MaybeDefault<FieldDecoder<F5, TolerableFaultsDecoder>>,
            MaybeDefault<FieldDecoder<F6, DataFragmentCountDecoder>>,
        )>,
    >,
}

impl_message_decode!(DispersedBucketDecoder, DispersedBucket, |t: (
    BucketId,
    _,
    DeviceId,
    _,
    _,
    _,
)| {
    Ok(DispersedBucket {
        id: t.0.clone(),
        seqno: t.1,
        device: t.2.clone(),
        segment_count: t.3,
        tolerable_faults: t.4,
        data_fragment_count: t.5,
    })
});

/// Encoder for `DispersedBucket`.
#[derive(Debug, Default)]
pub struct DispersedBucketEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, SequenceNumberEncoder>,
            FieldEncoder<F3, DeviceIdEncoder>,
            FieldEncoder<F4, SegmentCountEncoder>,
            FieldEncoder<F5, TolerableFaultsEncoder>,
            FieldEncoder<F6, DataFragmentCountEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(
    DispersedBucketEncoder,
    DispersedBucket,
    |item: Self::Item| {
        (
            item.id,
            item.seqno,
            item.device,
            item.segment_count,
            item.tolerable_faults,
            item.data_fragment_count,
        )
    }
);
