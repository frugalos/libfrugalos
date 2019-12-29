//! Decoders and encoders for [libfrugalos.deadline].

use bytecodec::{ErrorKind, Result};
use protobuf_codec::wellknown::google::protobuf::{
    DurationMessage, DurationMessageDecoder, DurationMessageEncoder,
};
use std::time::Duration;
use trackable::error::ErrorKindExt;

/// Decoder for `Deadline`.
// 互換性に注意: frugalos では秒で扱っている
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L98
pub type DeadlineDecoder = DurationMessageDecoder;

/// Encoder for `Deadline`.
// 互換性に注意: frugalos では秒で扱っている
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L109
pub type DeadlineEncoder = DurationMessageEncoder;

/// Decodes `Deadline`.
pub fn decode_deadline(deadline: DurationMessage) -> Result<Duration> {
    track!(deadline.to_duration().ok_or_else(|| ErrorKind::InvalidInput
        .cause(format!("incorrect duration: {:?}", deadline))
        .into()))
}

/// Encodes `Deadline`.
pub fn encode_deadline(deadline: Duration) -> DurationMessage {
    // FIXME: Result を返すようにする
    DurationMessage::from_duration(deadline).unwrap_or_else(|e| unreachable!("{:?}", e))
}
