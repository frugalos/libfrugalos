//! test

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5, F6};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Optional, Repeated,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    BoolDecoder, BoolEncoder, BytesDecoder, BytesEncoder, StringDecoder, StringEncoder,
    Uint32Decoder, Uint32Encoder,
};

use entity::bucket::BucketId;
use entity::device::DeviceId;
use entity::object::{FragmentsSummary, ObjectId, ObjectVersion};
use protobuf::consistency::{ReadConsistencyDecoder, ReadConsistencyEncoder};
use protobuf::deadline::{decode_deadline, encode_deadline, DeadlineDecoder, DeadlineEncoder};
use protobuf::entity::bucket::{BucketIdDecoder, BucketIdEncoder};
use protobuf::entity::device::{DeviceIdDecoder, DeviceIdEncoder};
use protobuf::entity::object::{
    FragmentsSummaryDecoder, FragmentsSummaryEncoder, ObjectIdDecoder, ObjectIdEncoder,
    ObjectPrefixDecoder, ObjectPrefixEncoder, ObjectRangeDecoder, ObjectRangeEncoder,
    ObjectVersionDecoder, ObjectVersionEncoder,
};
use protobuf::expect::{ExpectDecoder, ExpectEncoder};
use protobuf::{OptionDecoder, OptionEncoder, ResultDecoder, ResultEncoder};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};
use schema::frugalos::{
    CountFragmentsRequest, DeleteObjectSetFromDeviceRequest, HeadObjectRequest, ListObjectsRequest,
    ObjectRequest, PrefixRequest, PutObjectRequest, RangeRequest, SegmentRequest, VersionRequest,
};
use Result;

/// Decoder for `ObjectRequest`.
#[derive(Debug, Default)]
pub struct ObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MessageFieldDecoder<F3, DeadlineDecoder>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            Optional<MessageFieldDecoder<F5, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ObjectRequestDecoder, ObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.2))?;
    Ok(ObjectRequest {
        bucket_id: t.0.clone(),
        object_id: t.1.clone(),
        deadline,
        expect: t.3,
        consistency: t.4,
    })
});

/// Encoder for `ObjectRequest`.
#[derive(Debug, Default)]
pub struct ObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, StringEncoder>,
            MessageFieldEncoder<F3, DeadlineEncoder>,
            MessageFieldEncoder<F4, PreEncode<ExpectEncoder>>,
            Optional<MessageFieldEncoder<F5, ReadConsistencyEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(ObjectRequestEncoder, ObjectRequest, |item: Self::Item| {
    (
        item.bucket_id,
        item.object_id,
        encode_deadline(item.deadline),
        item.expect,
        item.consistency,
    )
});

/// Decoder for `CountFragmentsRequest`.
#[derive(Debug, Default)]
pub struct CountFragmentsRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectIdDecoder>>,
            MessageFieldDecoder<F3, DeadlineDecoder>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            MessageFieldDecoder<F5, ReadConsistencyDecoder>,
        )>,
    >,
}
impl_message_decode!(
    CountFragmentsRequestDecoder,
    CountFragmentsRequest,
    |t: (String, String, _, _, _,)| {
        let deadline = track!(decode_deadline(t.2))?;
        Ok(CountFragmentsRequest {
            bucket_id: t.0.clone(),
            object_id: t.1.clone(),
            deadline,
            expect: t.3,
            consistency: t.4,
        })
    }
);

/// Encoder for `CountFragmentsRequest`.
#[derive(Debug, Default)]
pub struct CountFragmentsRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, ObjectIdEncoder>,
            MessageFieldEncoder<F3, DeadlineEncoder>,
            MessageFieldEncoder<F4, PreEncode<ExpectEncoder>>,
            MessageFieldEncoder<F5, ReadConsistencyEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    CountFragmentsRequestEncoder,
    CountFragmentsRequest,
    |item: Self::Item| {
        (
            item.bucket_id,
            item.object_id,
            encode_deadline(item.deadline),
            item.expect,
            item.consistency,
        )
    }
);

/// Decoder for `HeadObjectRequest`.
#[derive(Debug, Default)]
pub struct HeadObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectIdDecoder>>,
            MessageFieldDecoder<F3, DeadlineDecoder>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            MessageFieldDecoder<F5, ReadConsistencyDecoder>,
            MaybeDefault<FieldDecoder<F6, BoolDecoder>>,
        )>,
    >,
}
impl_message_decode!(HeadObjectRequestDecoder, HeadObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.2))?;
    Ok(HeadObjectRequest {
        bucket_id: t.0.clone(),
        object_id: t.1.clone(),
        deadline,
        expect: t.3,
        consistency: t.4,
        check_storage: t.5,
    })
});

/// Encoder for `HeadObjectRequest`.
#[derive(Debug, Default)]
pub struct HeadObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, ObjectIdEncoder>,
            MessageFieldEncoder<F3, DeadlineEncoder>,
            MessageFieldEncoder<F4, PreEncode<ExpectEncoder>>,
            MessageFieldEncoder<F5, ReadConsistencyEncoder>,
            FieldEncoder<F6, BoolEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    HeadObjectRequestEncoder,
    HeadObjectRequest,
    |item: Self::Item| {
        (
            item.bucket_id,
            item.object_id,
            encode_deadline(item.deadline),
            item.expect,
            item.consistency,
            item.check_storage,
        )
    }
);

/// Decoder for `VersionRequest`.
#[derive(Debug, Default)]
pub struct VersionRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, ObjectVersionDecoder>>,
            MessageFieldDecoder<F4, DeadlineDecoder>,
        )>,
    >,
}
impl_message_decode!(VersionRequestDecoder, VersionRequest, |t: (
    String,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.3))?;
    Ok(VersionRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
        object_version: ObjectVersion(t.2),
        deadline,
    })
});

/// Encoder for `VersionRequest`.
#[derive(Debug, Default)]
pub struct VersionRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, ObjectVersionEncoder>,
            MessageFieldEncoder<F4, DeadlineEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(VersionRequestEncoder, VersionRequest, |item: Self::Item| {
    (
        item.bucket_id,
        item.segment as u32,
        item.object_version.0,
        encode_deadline(item.deadline),
    )
});

/// Decoder for `RangeRequest`.
#[derive(Debug, Default)]
pub struct RangeRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MessageFieldDecoder<F3, ObjectRangeDecoder>,
            MessageFieldDecoder<F4, DeadlineDecoder>,
        )>,
    >,
}
impl_message_decode!(RangeRequestDecoder, RangeRequest, |t: (String, _, _, _,)| {
    let deadline = track!(decode_deadline(t.3))?;
    Ok(RangeRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
        targets: t.2,
        deadline,
    })
});

/// Encoder for `RangeRequest`.
#[derive(Debug, Default)]
pub struct RangeRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            MessageFieldEncoder<F3, ObjectRangeEncoder>,
            MessageFieldEncoder<F4, DeadlineEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(RangeRequestEncoder, RangeRequest, |item: Self::Item| {
    (
        item.bucket_id,
        item.segment as u32,
        item.targets,
        encode_deadline(item.deadline),
    )
});

/// Decoder for `PrefixRequest`.
#[derive(Debug, Default)]
pub struct PrefixRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MessageFieldDecoder<F2, ObjectPrefixDecoder>,
            MessageFieldDecoder<F3, DeadlineDecoder>,
        )>,
    >,
}
impl_message_decode!(PrefixRequestDecoder, PrefixRequest, |t: (String, _, _,)| {
    let deadline = track!(decode_deadline(t.2))?;
    Ok(PrefixRequest {
        bucket_id: t.0.clone(),
        prefix: t.1,
        deadline,
    })
});

/// Encoder for `PrefixRequest`.
#[derive(Debug, Default)]
pub struct PrefixRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            MessageFieldEncoder<F2, ObjectPrefixEncoder>,
            MessageFieldEncoder<F3, DeadlineEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(PrefixRequestEncoder, PrefixRequest, |item: Self::Item| {
    (item.bucket_id, item.prefix, encode_deadline(item.deadline))
});

/// Decoder for `PutObjectRequest`.
#[derive(Debug, Default)]
pub struct PutObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectIdDecoder>>,
            MaybeDefault<FieldDecoder<F3, BytesDecoder>>,
            MessageFieldDecoder<F4, DeadlineDecoder>,
            MessageFieldDecoder<F5, ExpectDecoder>,
            FieldDecoder<F6, Uint32Decoder>, // TODO
        )>,
    >,
}
impl_message_decode!(PutObjectRequestDecoder, PutObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.3))?;
    Ok(PutObjectRequest {
        bucket_id: t.0.clone(),
        object_id: t.1.clone(),
        content: t.2,
        deadline,
        expect: t.4,
        multiplicity_config: Default::default(), // TODO
    })
});

/// Encoder for `PutObjectRequest`.
#[derive(Debug, Default)]
pub struct PutObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, ObjectIdEncoder>,
            FieldEncoder<F3, BytesEncoder>,
            MessageFieldEncoder<F4, DeadlineEncoder>,
            MessageFieldEncoder<F5, PreEncode<ExpectEncoder>>,
            FieldEncoder<F6, Uint32Encoder>, // TODO
        )>,
    >,
}
impl_sized_message_encode!(
    PutObjectRequestEncoder,
    PutObjectRequest,
    |item: Self::Item| {
        (
            item.bucket_id,
            item.object_id,
            item.content,
            encode_deadline(item.deadline),
            item.expect,
            Default::default(), // TODO
        )
    }
);

/// Decoder for `ListObjectsRequestEncoder`.
#[derive(Debug, Default)]
pub struct ListObjectsRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<MessageFieldDecoder<F3, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ListObjectsRequestDecoder, ListObjectsRequest, |t: (
    String,
    _,
    _
)| {
    Ok(ListObjectsRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
        consistency: t.2,
    })
});

/// Encoder for `ListObjectsRequestEncoder`.
#[derive(Debug, Default)]
pub struct ListObjectsRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ListObjectsRequestEncoder,
    ListObjectsRequest,
    |t: Self::Item| { (t.bucket_id, t.segment as u32) }
);

/// Decoder for `SegmentRequest`.
#[derive(Debug, Default)]
pub struct SegmentRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            FieldDecoder<F1, BucketIdDecoder>,
            FieldDecoder<F2, Uint32Decoder>,
        )>,
    >,
}
impl_message_decode!(SegmentRequestDecoder, SegmentRequest, |t: (String, u32)| {
    Ok(SegmentRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
    })
});

/// Encoder for `SegmentRequest`.
#[derive(Debug, Default)]
pub struct SegmentRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(SegmentRequestEncoder, SegmentRequest, |t: Self::Item| {
    (t.bucket_id, t.segment as u32)
});

/// Decoder for `DeleteObjectSetFromDeviceRequest`.
#[derive(Debug, Default)]
pub struct DeleteObjectSetFromDeviceRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, DeviceIdDecoder>>,
            Repeated<FieldDecoder<F3, ObjectIdDecoder>, Vec<String>>,
        )>,
    >,
}
impl_message_decode!(
    DeleteObjectSetFromDeviceRequestDecoder,
    DeleteObjectSetFromDeviceRequest,
    |t: (BucketId, DeviceId, Vec<ObjectId>,)| {
        Ok(DeleteObjectSetFromDeviceRequest {
            bucket_id: t.0.clone(),
            device_id: t.1.clone(),
            object_ids: t.2.into_iter().collect(),
        })
    }
);

/// Encoder for `DeleteObjectSetFromDeviceRequest`.
#[derive(Debug, Default)]
pub struct DeleteObjectSetFromDeviceRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, DeviceIdEncoder>,
            Repeated<FieldEncoder<F3, ObjectIdEncoder>, Vec<String>>,
        )>,
    >,
}
impl_message_encode!(
    DeleteObjectSetFromDeviceRequestEncoder,
    DeleteObjectSetFromDeviceRequest,
    |item: Self::Item| {
        (
            item.bucket_id,
            item.device_id,
            item.object_ids.into_iter().collect(),
        )
    }
);

/// Decoder for a response of `GetObject`.
#[derive(Debug, Default)]
pub struct GetObjectResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<
                OptionDecoder<
                    MessageDecoder<
                        Fields<(
                            MaybeDefault<FieldDecoder<F1, ObjectVersionDecoder>>,
                            MaybeDefault<FieldDecoder<F2, BytesDecoder>>,
                        )>,
                    >,
                >,
            >,
        >,
    >,
}
impl_message_decode!(
    GetObjectResponseDecoder,
    Result<Option<(ObjectVersion, Vec<u8>)>>,
    |r: Result<Option<(u64, Vec<u8>)>>| Ok(r.map(|v| v.map(|t| (ObjectVersion(t.0), t.1))))
);

/// Encoder for a response of `GetObject`.
#[derive(Debug, Default)]
pub struct GetObjectResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<
                OptionEncoder<
                    MessageEncoder<
                        Fields<(
                            FieldEncoder<F1, ObjectVersionEncoder>,
                            FieldEncoder<F2, BytesEncoder>,
                        )>,
                    >,
                >,
            >,
        >,
    >,
}
impl_sized_message_encode!(
    GetObjectResponseEncoder,
    Result<Option<(ObjectVersion, Vec<u8>)>>,
    |item: Self::Item| item.map(|v| v.map(|t| ((t.0).0, t.1)))
);

/// Decoder for a response of `PutObject`.
#[derive(Debug, Default)]
pub struct PutObjectResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<
                MessageDecoder<
                    Fields<(
                        MaybeDefault<FieldDecoder<F1, ObjectVersionDecoder>>,
                        MaybeDefault<FieldDecoder<F2, BoolDecoder>>,
                    )>,
                >,
            >,
        >,
    >,
}
impl_message_decode!(
    PutObjectResponseDecoder,
    Result<(ObjectVersion, bool)>,
    |r: Result<(u64, bool)>| Ok(r.map(|t| (ObjectVersion(t.0), t.1)))
);

/// Encoder for a response of `PutObject`.
#[derive(Debug, Default)]
pub struct PutObjectResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<
                MessageEncoder<
                    Fields<(
                        FieldEncoder<F1, ObjectVersionEncoder>,
                        FieldEncoder<F2, BoolEncoder>,
                    )>,
                >,
            >,
        >,
    >,
}
impl_sized_message_encode!(
    PutObjectResponseEncoder,
    Result<(ObjectVersion, bool)>,
    |item: Self::Item| item.map(|t| ((t.0).0, t.1))
);

/// Decoder for a response of `CountFragments`.
#[derive(Debug, Default)]
pub struct CountFragmentsResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<FragmentsSummaryDecoder>>>,
    >,
}
impl_message_decode!(
    CountFragmentsResponseDecoder,
    Result<Option<FragmentsSummary>>,
    |r: Result<Option<FragmentsSummary>>| Ok(r)
);

/// Encoder for a response of `CountFragments`.
#[derive(Debug, Default)]
pub struct CountFragmentsResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<FragmentsSummaryEncoder>>>,
    >,
}
impl_sized_message_encode!(
    CountFragmentsResponseEncoder,
    Result<Option<FragmentsSummary>>,
    |item: Self::Item| item
);

/// Decoder for a response of `Stop`.
#[derive(Debug, Default)]
pub struct StopResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<EmptyMessageDecoder>>>,
}
impl_message_decode!(StopResponseDecoder, Result<()>, |r: _| Ok(r));

/// Encoder for a response of `Stop`.
#[derive(Debug, Default)]
pub struct StopResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<EmptyMessageEncoder>>>,
}
impl_sized_message_encode!(StopResponseEncoder, Result<()>, |item: Self::Item| item);

/// Decoder for a response of `TakeSnapshot`.
#[derive(Debug, Default)]
pub struct TakeSnapshotResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<EmptyMessageDecoder>>>,
}
impl_message_decode!(TakeSnapshotResponseDecoder, Result<()>, |r: _| Ok(r));

/// Encoder for a response of `TakeSnapshot`.
#[derive(Debug, Default)]
pub struct TakeSnapshotResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<EmptyMessageEncoder>>>,
}
impl_sized_message_encode!(
    TakeSnapshotResponseEncoder,
    Result<()>,
    |item: Self::Item| item
);

/// Decoder for a response of `Empty`.
#[derive(Debug, Default)]
pub struct EmptyResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<EmptyMessageDecoder>>>,
}
impl_message_decode!(EmptyResponseDecoder, Result<()>, |r: _| Ok(r));

/// Encoder for a response of `Empty`.
#[derive(Debug, Default)]
pub struct EmptyResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<EmptyMessageEncoder>>>,
}
impl_sized_message_encode!(EmptyResponseEncoder, Result<()>, |item: Self::Item| item);
