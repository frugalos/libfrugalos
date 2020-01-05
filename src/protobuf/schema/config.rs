//! Decoders and encoders for [`libfrugalos::schema::config`](../../schema/config/index.html).
//!
//! `package libfrugalos.protobuf.schema.config`.

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::num::F1;
use protobuf_codec::field::{MessageFieldDecoder, MessageFieldEncoder};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use std::net::SocketAddr;

use entity::bucket::{Bucket, BucketSummary};
use entity::device::{Device, DeviceSummary};
use entity::server::{Server, ServerSummary};
use protobuf::entity::bucket::{
    BucketDecoder, BucketEncoder, BucketSummaryDecoder, BucketSummaryEncoder,
};
use protobuf::entity::device::{
    DeviceDecoder, DeviceEncoder, DeviceSummaryDecoder, DeviceSummaryEncoder,
};
use protobuf::entity::server::{
    ServerDecoder, ServerEncoder, ServerSummaryDecoder, ServerSummaryEncoder,
};
use protobuf::net::{SocketAddrDecoder, SocketAddrEncoder};
use protobuf::{
    OptionDecoder, OptionEncoder, ResultDecoder, ResultEncoder, VecDecoder, VecEncoder,
};
use Result;

/// Decoder for `ListServers`.
#[derive(Debug, Default)]
pub struct ListServersResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<ServerSummaryDecoder>>>>,
}
impl_message_decode!(
    ListServersResponseDecoder,
    Result<Vec<ServerSummary>>,
    |t: _| Ok(t)
);

/// Encoder for `ListServers`.
#[derive(Debug, Default)]
pub struct ListServersResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<ServerSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    ListServersResponseEncoder,
    Result<Vec<ServerSummary>>,
    |item: Self::Item| item
);

/// Decoder for `GetServer`.
#[derive(Debug, Default)]
pub struct GetServerResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<ServerDecoder>>>>,
}
impl_message_decode!(GetServerResponseDecoder, Result<Option<Server>>, |t: _| Ok(
    t
));

/// Encoder for `GetServer`.
#[derive(Debug, Default)]
pub struct GetServerResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<ServerEncoder>>>>,
}
impl_sized_message_encode!(
    GetServerResponseEncoder,
    Result<Option<Server>>,
    |item: Self::Item| item
);

/// Decoder for `PutServer`.
#[derive(Debug, Default)]
pub struct PutServerResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<ServerDecoder>>>,
}
impl_message_decode!(PutServerResponseDecoder, Result<Server>, |t: _| Ok(t));

/// Encoder for `PutServer`.
#[derive(Debug, Default)]
pub struct PutServerResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<ServerEncoder>>>,
}
impl_sized_message_encode!(
    PutServerResponseEncoder,
    Result<Server>,
    |item: Self::Item| item
);

/// Decoder for `DeleteServer`.
#[derive(Debug, Default)]
pub struct DeleteServerResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<ServerDecoder>>>>,
}
impl_message_decode!(
    DeleteServerResponseDecoder,
    Result<Option<Server>>,
    |t: _| Ok(t)
);

/// Encoder for `DeleteServer`.
#[derive(Debug, Default)]
pub struct DeleteServerResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<ServerEncoder>>>>,
}
impl_sized_message_encode!(
    DeleteServerResponseEncoder,
    Result<Option<Server>>,
    |item: Self::Item| item
);

/// Decoder for `ListDevices`.
#[derive(Debug, Default)]
pub struct ListDevicesResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<DeviceSummaryDecoder>>>>,
}
impl_message_decode!(
    ListDevicesResponseDecoder,
    Result<Vec<DeviceSummary>>,
    |t: _| Ok(t)
);

/// Encoder for `ListDevices`.
#[derive(Debug, Default)]
pub struct ListDevicesResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<DeviceSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    ListDevicesResponseEncoder,
    Result<Vec<DeviceSummary>>,
    |item: Self::Item| item
);

/// Decoder for `GetDevice`.
#[derive(Debug, Default)]
pub struct GetDeviceResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<DeviceDecoder>>>>,
}
impl_message_decode!(GetDeviceResponseDecoder, Result<Option<Device>>, |t: _| Ok(
    t
));

/// Encoder for `GetDevice`.
#[derive(Debug, Default)]
pub struct GetDeviceResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<DeviceEncoder>>>>,
}
impl_sized_message_encode!(
    GetDeviceResponseEncoder,
    Result<Option<Device>>,
    |item: Self::Item| item
);

/// Decoder for `PutDevice`.
#[derive(Debug, Default)]
pub struct PutDeviceResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<DeviceDecoder>>>,
}
impl_message_decode!(PutDeviceResponseDecoder, Result<Device>, |t: _| Ok(t));

/// Encoder for `PutDevice`.
#[derive(Debug, Default)]
pub struct PutDeviceResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<DeviceEncoder>>>,
}
impl_sized_message_encode!(
    PutDeviceResponseEncoder,
    Result<Device>,
    |item: Self::Item| item
);

/// Decoder for `DeleteDevice`.
#[derive(Debug, Default)]
pub struct DeleteDeviceResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<DeviceDecoder>>>>,
}
impl_message_decode!(
    DeleteDeviceResponseDecoder,
    Result<Option<Device>>,
    |t: _| Ok(t)
);

/// Encoder for `DeleteDevice`.
#[derive(Debug, Default)]
pub struct DeleteDeviceResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<DeviceEncoder>>>>,
}
impl_sized_message_encode!(
    DeleteDeviceResponseEncoder,
    Result<Option<Device>>,
    |item: Self::Item| item
);

/// Decoder for `ListBuckets`.
#[derive(Debug, Default)]
pub struct ListBucketsResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<BucketSummaryDecoder>>>>,
}
impl_message_decode!(
    ListBucketsResponseDecoder,
    Result<Vec<BucketSummary>>,
    |t: _| Ok(t)
);

/// Encoder for `ListBuckets`.
#[derive(Debug, Default)]
pub struct ListBucketsResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<BucketSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    ListBucketsResponseEncoder,
    Result<Vec<BucketSummary>>,
    |item: Self::Item| item
);

/// Decoder for `GetBucket`.
#[derive(Debug, Default)]
pub struct GetBucketResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<BucketDecoder>>>>,
}
impl_message_decode!(GetBucketResponseDecoder, Result<Option<Bucket>>, |t: _| Ok(
    t
));

/// Encoder for `GetBucket`.
#[derive(Debug, Default)]
pub struct GetBucketResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<BucketEncoder>>>>,
}
impl_sized_message_encode!(
    GetBucketResponseEncoder,
    Result<Option<Bucket>>,
    |item: Self::Item| item
);

/// Decoder for `PutBucket`.
#[derive(Debug, Default)]
pub struct PutBucketResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<BucketDecoder>>>,
}
impl_message_decode!(PutBucketResponseDecoder, Result<Bucket>, |t: _| Ok(t));

/// Encoder for `PutBucket`.
#[derive(Debug, Default)]
pub struct PutBucketResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<BucketEncoder>>>,
}
impl_sized_message_encode!(
    PutBucketResponseEncoder,
    Result<Bucket>,
    |item: Self::Item| item
);

/// Decoder for `DeleteBucket`.
#[derive(Debug, Default)]
pub struct DeleteBucketResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<BucketDecoder>>>>,
}
impl_message_decode!(
    DeleteBucketResponseDecoder,
    Result<Option<Bucket>>,
    |t: _| Ok(t)
);

/// Encoder for `DeleteBucket`.
#[derive(Debug, Default)]
pub struct DeleteBucketResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<BucketEncoder>>>>,
}
impl_sized_message_encode!(
    DeleteBucketResponseEncoder,
    Result<Option<Bucket>>,
    |item: Self::Item| item
);

/// Decoder for `GetLeader`.
#[derive(Debug, Default)]
pub struct GetLeaderResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<SocketAddrDecoder>>>,
}
impl_message_decode!(GetLeaderResponseDecoder, Result<SocketAddr>, |t: _| Ok(t));

/// Encoder for `GetLeader`.
#[derive(Debug, Default)]
pub struct GetLeaderResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<SocketAddrEncoder>>>,
}
impl_sized_message_encode!(
    GetLeaderResponseEncoder,
    Result<SocketAddr>,
    |item: Self::Item| item
);
