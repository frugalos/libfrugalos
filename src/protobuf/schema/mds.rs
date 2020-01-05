//! Decoders and encoders for [`libmds::schema::mds`](../../schema/mds/index.html).
//!
//! `package libmds.protobuf.schema.mds`.

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Optional,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    BytesDecoder, BytesEncoder, StringDecoder, StringEncoder, Uint64Decoder, Uint64Encoder,
};
use std::time::Duration;

use entity::node::RemoteNodeId;
use entity::object::{Metadata, ObjectVersion};
use protobuf::consistency::{ReadConsistencyDecoder, ReadConsistencyEncoder};
use protobuf::entity::node::{
    LocalNodeIdDecoder, LocalNodeIdEncoder, RemoteNodeIdDecoder, RemoteNodeIdEncoder,
};
use protobuf::entity::object::{
    MetadataDecoder, MetadataEncoder, ObjectPrefixDecoder, ObjectPrefixEncoder, ObjectRangeDecoder,
    ObjectRangeEncoder, ObjectVersionDecoder, ObjectVersionEncoder,
};
use protobuf::expect::{ExpectDecoder, ExpectEncoder};
use protobuf::{OptionDecoder, OptionEncoder, ResultDecoder, ResultEncoder};
use schema::mds::{
    GetLatestVersionRequest, GetLeaderRequest, ListObjectsRequest, ObjectCountRequest,
    ObjectRequest, PrefixRequest, PutObjectRequest, RangeRequest, RecommendToLeaderRequest,
    VersionRequest,
};
use Result;

/// Decoder for `GetLeaderRequest`.
#[derive(Debug, Default)]
pub struct GetLeaderRequestDecoder {
    inner: MessageDecoder<MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>>,
}
impl_message_decode!(GetLeaderRequestDecoder, GetLeaderRequest, |node_id: _| {
    Ok(GetLeaderRequest { node_id })
});

/// Encoder for `GetLeaderRequest`.
#[derive(Debug, Default)]
pub struct GetLeaderRequestEncoder {
    inner: MessageEncoder<FieldEncoder<F1, LocalNodeIdEncoder>>,
}
impl_sized_message_encode!(
    GetLeaderRequestEncoder,
    GetLeaderRequest,
    |item: Self::Item| item.node_id
);

/// Decoder for `RecommendToLeaderRequest`.
#[derive(Debug, Default)]
pub struct RecommendToLeaderRequestDecoder {
    inner: MessageDecoder<MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>>,
}
impl_message_decode!(
    RecommendToLeaderRequestDecoder,
    RecommendToLeaderRequest,
    |node_id: _| { Ok(RecommendToLeaderRequest { node_id }) }
);

/// Encoder for `RecommendToLeaderRequest`.
#[derive(Debug, Default)]
pub struct RecommendToLeaderRequestEncoder {
    inner: MessageEncoder<FieldEncoder<F1, LocalNodeIdEncoder>>,
}
impl_sized_message_encode!(
    RecommendToLeaderRequestEncoder,
    RecommendToLeaderRequest,
    |item: Self::Item| item.node_id
);

/// Decoder for `GetLatestVersionRequest`.
#[derive(Debug, Default)]
pub struct GetLatestVersionRequestDecoder {
    inner: MessageDecoder<MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>>,
}
impl_message_decode!(
    GetLatestVersionRequestDecoder,
    GetLatestVersionRequest,
    |node_id: _| { Ok(GetLatestVersionRequest { node_id }) }
);

/// Encoder for `GetLatestVersionRequest`.
#[derive(Debug, Default)]
pub struct GetLatestVersionRequestEncoder {
    inner: MessageEncoder<FieldEncoder<F1, LocalNodeIdEncoder>>,
}
impl_sized_message_encode!(
    GetLatestVersionRequestEncoder,
    GetLatestVersionRequest,
    |item: Self::Item| item.node_id
);

/// Decoder for `ObjectRequest`.
#[derive(Debug, Default)]
pub struct ObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MessageFieldDecoder<F3, ExpectDecoder>,
            Optional<MessageFieldDecoder<F4, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ObjectRequestDecoder, ObjectRequest, |t: (
    String,
    String,
    _,
    _,
)| {
    Ok(ObjectRequest {
        node_id: t.0.clone(),
        object_id: t.1.clone(),
        expect: t.2,
        consistency: t.3,
    })
});

/// Encoder for `ObjectRequest`.
#[derive(Debug, Default)]
pub struct ObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            FieldEncoder<F2, StringEncoder>,
            MessageFieldEncoder<F3, PreEncode<ExpectEncoder>>,
            Optional<MessageFieldEncoder<F4, ReadConsistencyEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(ObjectRequestEncoder, ObjectRequest, |item: Self::Item| {
    (item.node_id, item.object_id, item.expect, item.consistency)
});

/// Decoder for `ListObjectsRequest`.
#[derive(Debug, Default)]
pub struct ListObjectsRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ReadConsistencyDecoder>,
        )>,
    >,
}
impl_message_decode!(ListObjectsRequestDecoder, ListObjectsRequest, |t: (
    String,
    _,
)| {
    Ok(ListObjectsRequest {
        node_id: t.0.clone(),
        consistency: t.1,
    })
});

/// Encoder for `ListObjectsRequest`.
#[derive(Debug, Default)]
pub struct ListObjectsRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ReadConsistencyEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ListObjectsRequestEncoder,
    ListObjectsRequest,
    |item: Self::Item| { (item.node_id, item.consistency,) }
);

/// Decoder for `ObjectCountRequest`.
#[derive(Debug, Default)]
pub struct ObjectCountRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ReadConsistencyDecoder>,
        )>,
    >,
}
impl_message_decode!(ObjectCountRequestDecoder, ObjectCountRequest, |t: (
    String,
    _,
)| {
    Ok(ObjectCountRequest {
        node_id: t.0.clone(),
        consistency: t.1,
    })
});

/// Encoder for `ObjectCountRequest`.
#[derive(Debug, Default)]
pub struct ObjectCountRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ReadConsistencyEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ObjectCountRequestEncoder,
    ObjectCountRequest,
    |item: Self::Item| { (item.node_id, item.consistency,) }
);

/// Decoder for `VersionRequest`.
#[derive(Debug, Default)]
pub struct VersionRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectVersionDecoder>>,
        )>,
    >,
}
impl_message_decode!(VersionRequestDecoder, VersionRequest, |t: (String, _,)| {
    Ok(VersionRequest {
        node_id: t.0.clone(),
        object_version: ObjectVersion(t.1),
    })
});

/// Encoder for `VersionRequest`.
#[derive(Debug, Default)]
pub struct VersionRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            FieldEncoder<F2, ObjectVersionEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(VersionRequestEncoder, VersionRequest, |item: Self::Item| {
    (item.node_id, item.object_version.0)
});

/// Decoder for `RangeRequest`.
#[derive(Debug, Default)]
pub struct RangeRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ObjectRangeDecoder>,
        )>,
    >,
}
impl_message_decode!(RangeRequestDecoder, RangeRequest, |t: (String, _,)| {
    Ok(RangeRequest {
        node_id: t.0.clone(),
        targets: t.1,
    })
});

/// Encoder for `RangeRequest`.
#[derive(Debug, Default)]
pub struct RangeRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ObjectRangeEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(RangeRequestEncoder, RangeRequest, |item: Self::Item| {
    (item.node_id, item.targets)
});

/// Decoder for `PrefixRequest`.
#[derive(Debug, Default)]
pub struct PrefixRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ObjectPrefixDecoder>,
        )>,
    >,
}
impl_message_decode!(PrefixRequestDecoder, PrefixRequest, |t: (String, _,)| {
    Ok(PrefixRequest {
        node_id: t.0.clone(),
        prefix: t.1,
    })
});

/// Encoder for `PrefixRequest`.
#[derive(Debug, Default)]
pub struct PrefixRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ObjectPrefixEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(PrefixRequestEncoder, PrefixRequest, |item: Self::Item| {
    (item.node_id, item.prefix)
});

/// Decoder for `PutObjectRequest`.
#[derive(Debug, Default)]
pub struct PutObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MaybeDefault<FieldDecoder<F3, BytesDecoder>>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            FieldDecoder<F5, Uint64Decoder>, // TODO
        )>,
    >,
}
impl_message_decode!(PutObjectRequestDecoder, PutObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
)| {
    Ok(PutObjectRequest {
        node_id: t.0.clone(),
        object_id: t.1.clone(),
        metadata: t.2,
        expect: t.3,
        put_content_timeout: Duration::from_secs(t.4),
    })
});

/// Encoder for `PutObjectRequest`.
#[derive(Debug, Default)]
pub struct PutObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            FieldEncoder<F2, StringEncoder>,
            FieldEncoder<F3, BytesEncoder>,
            MessageFieldEncoder<F4, ExpectEncoder>,
            FieldEncoder<F5, Uint64Encoder>, // TODO
        )>,
    >,
}
impl_sized_message_encode!(
    PutObjectRequestEncoder,
    PutObjectRequest,
    |item: Self::Item| {
        (
            item.node_id,
            item.object_id,
            item.metadata,
            item.expect,
            item.put_content_timeout.as_secs(),
        )
    }
);

/// Decoder for a response of [GetLeaderRpc].
#[derive(Debug, Default)]
pub struct GetLeaderResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<RemoteNodeIdDecoder>>>,
}
impl_message_decode!(GetLeaderResponseDecoder, Result<RemoteNodeId>, |t: _| Ok(t));

/// Encoder for a response of [GetLeaderRpc].
#[derive(Debug, Default)]
pub struct GetLeaderResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<RemoteNodeIdEncoder>>>,
}
impl_message_encode!(
    GetLeaderResponseEncoder,
    Result<RemoteNodeId>,
    |item: Self::Item| item
);

/// Decoder for a response of `Option<Metadata>`.
#[derive(Debug, Default)]
pub struct MaybeMetadataResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<MetadataDecoder>>>>,
}
impl_message_decode!(
    MaybeMetadataResponseDecoder,
    Result<Option<Metadata>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `Option<Metadata>`.
#[derive(Debug, Default)]
pub struct MaybeMetadataResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<MetadataEncoder>>>>,
}
impl_message_encode!(
    MaybeMetadataResponseEncoder,
    Result<Option<Metadata>>,
    |item: Self::Item| item
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
                        Optional<FieldDecoder<F2, ObjectVersionDecoder>>,
                    )>,
                >,
            >,
        >,
    >,
}
impl_message_decode!(
    PutObjectResponseDecoder,
    Result<(ObjectVersion, Option<ObjectVersion>)>,
    |r: Result<(u64, Option<u64>)>| Ok(r.map(|t| (ObjectVersion(t.0), t.1.map(ObjectVersion))))
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
                        Optional<FieldEncoder<F2, ObjectVersionEncoder>>,
                    )>,
                >,
            >,
        >,
    >,
}
impl_message_encode!(
    PutObjectResponseEncoder,
    Result<(ObjectVersion, Option<ObjectVersion>)>,
    |item: Self::Item| item.map(|t| ((t.0).0, t.1.map(|v| v.0)))
);

/// Decoder for a response of `ObjectCount`.
#[derive(Debug, Default)]
pub struct ObjectCountResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<F1, ResultDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>>,
    >,
}
impl_message_decode!(ObjectCountResponseDecoder, Result<u64>, |t: _| Ok(t));

/// Encoder for a response of `ObjectCount`.
#[derive(Debug, Default)]
pub struct ObjectCountResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, ResultEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>>,
    >,
}
impl_message_encode!(
    ObjectCountResponseEncoder,
    Result<u64>,
    |item: Self::Item| item
);
