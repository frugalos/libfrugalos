//! Decoders and encoders for [`libfrugalos::deadline`](../../deadline/index.html).
//!
//! `package libfrugalos.protobuf.deadline`.

use bytecodec::{ErrorKind, Result};
use protobuf_codec::wellknown::google::protobuf::{
    DurationMessage, DurationMessageDecoder, DurationMessageEncoder,
};
use std::time::Duration;
use trackable::error::ErrorKindExt;

/// Decoder for [`Deadline`](../../deadline/struct.Deadline.html).
// 互換性に注意: frugalos では秒で扱っている
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L98
pub type DeadlineDecoder = DurationMessageDecoder;

/// Encoder for [`Deadline`](../../deadline/struct.Deadline.html).
// 互換性に注意: frugalos では秒で扱っている
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L109
pub type DeadlineEncoder = DurationMessageEncoder;

/// Decodes [Deadline](../../deadline/struct.Deadline.html).
///
/// # Errors
///
/// Returns [`bytecodec::ErrorKind::InvalidInput`](../../../bytecodec/enum.ErrorKind.html#variant.InvalidInput)
/// if an input is not a valid duration.
///
/// See [`bytecodec::Error`](bytecodec/enum.ErrorKind.html).
pub fn decode_deadline(deadline: DurationMessage) -> Result<Duration> {
    track!(deadline.to_duration().ok_or_else(|| ErrorKind::InvalidInput
        .cause(format!("incorrect duration: {:?}", deadline))
        .into()))
}

/// Encodes [Deadline](../../deadline/struct.Deadline.html).
pub fn encode_deadline(deadline: Duration) -> DurationMessage {
    // FIXME: Result を返すようにする
    DurationMessage::from_duration(deadline).unwrap_or_else(|e| unreachable!("{:?}", e))
}
