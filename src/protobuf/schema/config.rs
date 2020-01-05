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

/// Decoder for `ListServersRpc`.
pub type ListServersResponseDecoder = ResultDecoder<VecDecoder<ServerSummaryDecoder>>;

/// Encoder for `ListServersRpc`.
pub type ListServersResponseEncoder = ResultEncoder<VecEncoder<ServerSummaryEncoder>>;

/// Decoder for `GetServerRpc`.
pub type GetServerResponseDecoder = ResultDecoder<OptionDecoder<ServerDecoder>>;

/// Encoder for `GetServerRpc`.
pub type GetServerResponseEncoder = ResultEncoder<OptionEncoder<ServerEncoder>>;

/// Decoder for `PutServerRpc`.
pub type PutServerResponseDecoder = ResultDecoder<ServerDecoder>;

/// Encoder for `PutServerRpc`.
pub type PutServerResponseEncoder = ResultEncoder<ServerEncoder>;

/// Decoder for `DeleteServerRpc`.
pub type DeleteServerResponseDecoder = ResultDecoder<OptionDecoder<ServerDecoder>>;

/// Encoder for `DeleteServerRpc`.
pub type DeleteServerResponseEncoder = ResultEncoder<OptionEncoder<ServerEncoder>>;

/// Decoder for `ListDevicesRpc`.
pub type ListDevicesResponseDecoder = ResultDecoder<VecDecoder<DeviceSummaryDecoder>>;

/// Encoder for `ListDevicesRpc`.
pub type ListDevicesResponseEncoder = ResultEncoder<VecEncoder<DeviceSummaryEncoder>>;

/// Decoder for `GetDeviceRpc`.
pub type GetDeviceResponseDecoder = ResultDecoder<OptionDecoder<DeviceDecoder>>;

/// Encoder for `GetDeviceRpc`.
pub type GetDeviceResponseEncoder = ResultEncoder<OptionEncoder<DeviceEncoder>>;

/// Decoder for `PutDeviceRpc`.
pub type PutDeviceResponseDecoder = ResultDecoder<DeviceDecoder>;

/// Encoder for `PutDeviceRpc`.
pub type PutDeviceResponseEncoder = ResultEncoder<DeviceEncoder>;

/// Decoder for `DeleteDeviceRpc`.
pub type DeleteDeviceResponseDecoder = ResultDecoder<OptionDecoder<DeviceDecoder>>;

/// Encoder for `DeleteDeviceRpc`.
pub type DeleteDeviceResponseEncoder = ResultEncoder<OptionEncoder<DeviceEncoder>>;

/// Decoder for `ListBucketsRpc`.
pub type ListBucketsResponseDecoder = ResultDecoder<VecDecoder<BucketSummaryDecoder>>;

/// Encoder for `ListBucketsRpc`.
pub type ListBucketsResponseEncoder = ResultEncoder<VecEncoder<BucketSummaryEncoder>>;

/// Decoder for `GetBucketRpc`.
pub type GetBucketResponseDecoder = ResultDecoder<OptionDecoder<BucketDecoder>>;

/// Encoder for `GetBucketRpc`.
pub type GetBucketResponseEncoder = ResultEncoder<OptionEncoder<BucketEncoder>>;

/// Decoder for `PutBucketRpc`.
pub type PutBucketResponseDecoder = ResultDecoder<BucketDecoder>;

/// Encoder for `PutBucketRpc`.
pub type PutBucketResponseEncoder = ResultEncoder<BucketEncoder>;

/// Decoder for `DeleteBucketRpc`.
pub type DeleteBucketResponseDecoder = ResultDecoder<OptionDecoder<BucketDecoder>>;

/// Encoder for `DeleteBucketRpc`.
pub type DeleteBucketResponseEncoder = ResultEncoder<OptionEncoder<BucketEncoder>>;

/// Decoder for `GetLeaderRpc`.
pub type GetLeaderResponseDecoder = ResultDecoder<SocketAddrDecoder>;

/// Encoder for `GetLeaderRpc`.
pub type GetLeaderResponseEncoder = ResultEncoder<SocketAddrEncoder>;
