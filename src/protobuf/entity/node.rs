//! Decoders and encoders for [`libfrugalos::entity::node`](../../entity/node/index.html).
//!
//! `package libfrugalos.protobuf.entity.node`.

use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Fields, MaybeDefault};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder};
use std::net::SocketAddr;
use std::str::FromStr;

use entity::node::RemoteNodeId;

/// Decoder for [`LocalNodeId`](../../../entity/node/type.LocalNodeId.html).
pub type LocalNodeIdDecoder = StringDecoder;

/// Encoder for [`LocalNodeId`](../../../entity/node/type.LocalNodeId.html).
pub type LocalNodeIdEncoder = StringEncoder;

/// Decoder for [`RemoteNodeId`](../../../entity/node/type.RemoteNodeId.html).
#[derive(Debug, Default)]
pub struct RemoteNodeIdDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
        )>,
    >,
}

impl_message_decode!(RemoteNodeIdDecoder, RemoteNodeId, |t: (String, String,)| {
    let addr = track_any_err!(SocketAddr::from_str(&t.0))?;
    Ok((addr, t.1))
});

/// Encoder for [`RemoteNodeId`](../../../entity/node/type.RemoteNodeId.html).
#[derive(Debug, Default)]
pub struct RemoteNodeIdEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, StringEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(RemoteNodeIdEncoder, RemoteNodeId, |item: Self::Item| {
    (item.0.to_string(), item.1)
});

#[cfg(test)]
mod tests {
    use super::*;
    use bytecodec::io::{IoDecodeExt, IoEncodeExt};
    use bytecodec::EncodeExt;
    use trackable::result::TestResult;

    use entity::node::LocalNodeId;

    #[test]
    fn encode_local_node_id_works() -> TestResult {
        let mut buf = Vec::new();
        let mut decoder = LocalNodeIdDecoder::default();
        let node_id: LocalNodeId = "1".into();
        let mut encoder = track!(LocalNodeIdEncoder::with_item(node_id.clone()))?;
        track!(encoder.encode_all(&mut buf))?;
        assert_eq!(buf, [1, 49]);
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!(node_id, message);
        Ok(())
    }

    #[test]
    fn encode_remote_node_id_works() -> TestResult {
        let mut buf = Vec::new();
        let mut decoder = RemoteNodeIdDecoder::default();
        let addr: SocketAddr = track_any_err!("127.0.0.1:8000".parse())?;
        let node_id: LocalNodeId = "0".into();
        let mut encoder = track!(RemoteNodeIdEncoder::with_item((addr, node_id.clone())))?;
        track!(encoder.encode_all(&mut buf))?;
        assert_eq!(
            buf,
            [10, 14, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 48, 48, 18, 1, 48]
        );
        let message = track!(decoder.decode_exact(&buf[..]))?;
        assert_eq!((addr, node_id), message);
        Ok(())
    }
}
