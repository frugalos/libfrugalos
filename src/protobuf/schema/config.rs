//! Decoders and encoders for [`libfrugalos::schema::config`](../../schema/config/index.html).
//!
//! `package libfrugalos.protobuf.schema.config`.

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

/// Decoder for `ListServers`.
pub type ListServersResponseDecoder = ResultDecoder<VecDecoder<ServerSummaryDecoder>>;

/// Encoder for `ListServers`.
pub type ListServersResponseEncoder = ResultEncoder<VecEncoder<ServerSummaryEncoder>>;

/// Decoder for `GetServer`.
pub type GetServerResponseDecoder = ResultDecoder<OptionDecoder<ServerDecoder>>;

/// Encoder for `GetServer`.
pub type GetServerResponseEncoder = ResultEncoder<OptionEncoder<ServerEncoder>>;

/// Decoder for `PutServer`.
pub type PutServerResponseDecoder = ResultDecoder<ServerDecoder>;

/// Encoder for `PutServer`.
pub type PutServerResponseEncoder = ResultEncoder<ServerEncoder>;

/// Decoder for `DeleteServer`.
pub type DeleteServerResponseDecoder = ResultDecoder<OptionDecoder<ServerDecoder>>;

/// Encoder for `DeleteServer`.
pub type DeleteServerResponseEncoder = ResultEncoder<OptionEncoder<ServerEncoder>>;

/// Decoder for `ListDevices`.
pub type ListDevicesResponseDecoder = ResultDecoder<VecDecoder<DeviceSummaryDecoder>>;

/// Encoder for `ListDevices`.
pub type ListDevicesResponseEncoder = ResultEncoder<VecEncoder<DeviceSummaryEncoder>>;

/// Decoder for `GetDevice`.
pub type GetDeviceResponseDecoder = ResultDecoder<OptionDecoder<DeviceDecoder>>;

/// Encoder for `GetDevice`.
pub type GetDeviceResponseEncoder = ResultEncoder<OptionEncoder<DeviceEncoder>>;

/// Decoder for `PutDevice`.
pub type PutDeviceResponseDecoder = ResultDecoder<DeviceDecoder>;

/// Encoder for `PutDevice`.
pub type PutDeviceResponseEncoder = ResultEncoder<DeviceEncoder>;

/// Decoder for `DeleteDevice`.
pub type DeleteDeviceResponseDecoder = ResultDecoder<OptionDecoder<DeviceDecoder>>;

/// Encoder for `DeleteDevice`.
pub type DeleteDeviceResponseEncoder = ResultEncoder<OptionEncoder<DeviceEncoder>>;

/// Decoder for `ListBuckets`.
pub type ListBucketsResponseDecoder = ResultDecoder<VecDecoder<BucketSummaryDecoder>>;

/// Encoder for `ListBuckets`.
pub type ListBucketsResponseEncoder = ResultEncoder<VecEncoder<BucketSummaryEncoder>>;

/// Decoder for `GetBucket`.
pub type GetBucketResponseDecoder = ResultDecoder<OptionDecoder<BucketDecoder>>;

/// Encoder for `GetBucket`.
pub type GetBucketResponseEncoder = ResultEncoder<OptionEncoder<BucketEncoder>>;

/// Decoder for `PutBucket`.
pub type PutBucketResponseDecoder = ResultDecoder<BucketDecoder>;

/// Encoder for `PutBucket`.
pub type PutBucketResponseEncoder = ResultEncoder<BucketEncoder>;

/// Decoder for `DeleteBucket`.
pub type DeleteBucketResponseDecoder = ResultDecoder<OptionDecoder<BucketDecoder>>;

/// Encoder for `DeleteBucket`.
pub type DeleteBucketResponseEncoder = ResultEncoder<OptionEncoder<BucketEncoder>>;

/// Decoder for `GetLeader`.
pub type GetLeaderResponseDecoder = ResultDecoder<SocketAddrDecoder>;

/// Encoder for `GetLeader`.
pub type GetLeaderResponseEncoder = ResultEncoder<SocketAddrEncoder>;
