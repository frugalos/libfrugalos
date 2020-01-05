//! Decoders and encoders for [`libfrugalos::Error`](../../struct.Error.html).
//!
//! `package libfrugalos.protobuf.error;`
use bytecodec::combinator::PreEncode;
use bytecodec::{ByteCount, Decode, Encode, Eos, Result};
use protobuf_codec::field::branch::Branch6;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5, F6};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof, Optional, Repeated,
};
use protobuf_codec::message::{MessageDecode, MessageDecoder, MessageEncode, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder};
use protobuf_codec::wellknown::google::protobuf::{EmptyMessageDecoder, EmptyMessageEncoder};
use protobuf_codec::wellknown::protobuf_codec::protobuf::trackable::{
    LocationDecoder, LocationEncoder,
};
use std::error::Error;
use trackable::error::{ErrorKindExt, TrackableError};
use trackable::{Location, Trackable};

use entity::object::ObjectVersion;
use protobuf::entity::object::{ObjectVersionDecoder, ObjectVersionEncoder};
use ErrorKind;

/// Decoder for [Error](../../struct.Error.html).
#[derive(Debug, Default)]
pub struct ErrorDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F1, ErrorKindDecoder>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            Repeated<MessageFieldDecoder<F3, LocationDecoder>, Vec<Location>>,
        )>,
    >,
}
impl ErrorDecoder {
    /// Makes a new `ErrorDecoder` instance.
    pub fn new() -> Self {
        Self::default()
    }
}
impl Decode for ErrorDecoder {
    type Item = TrackableError<ErrorKind>;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        track!(self.inner.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let (kind, cause, locations) = track!(self.inner.finish_decoding())?;
        let mut e = if cause.is_empty() {
            kind.error()
        } else {
            kind.cause(cause)
        };
        if let Some(h) = e.history_mut() {
            for l in locations {
                h.add(l);
            }
        }
        Ok(e)
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl MessageDecode for ErrorDecoder {}

/// Encoder for [Error](../../struct.Error.html).
#[derive(Debug, Default)]
pub struct ErrorEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F1, PreEncode<ErrorKindEncoder>>,
            MaybeDefault<FieldEncoder<F2, StringEncoder>>,
            Repeated<MessageFieldEncoder<F3, LocationEncoder>, Vec<Location>>,
        )>,
    >,
}
impl ErrorEncoder {
    /// Makes a new `ErrorEncoder` instance.
    pub fn new() -> Self {
        Self::default()
    }
}
impl Encode for ErrorEncoder {
    type Item = TrackableError<ErrorKind>;

    fn encode(&mut self, buf: &mut [u8], eos: Eos) -> Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> Result<()> {
        let item = (
            *item.kind(),
            item.source()
                .map(|e| e.to_string())
                .unwrap_or_else(String::new),
            item.history()
                .map(|h| h.events().to_owned())
                .unwrap_or_else(Vec::new),
        );
        track!(self.inner.start_encoding(item))
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl MessageEncode for ErrorEncoder {}

/// Decoder for [ErrorKind](../../enum.ErrorKind.html).
#[derive(Debug, Default)]
pub struct ErrorKindDecoder {
    inner: MessageDecoder<
        Oneof<(
            // [ErrorKind::InvalidInput]
            MessageFieldDecoder<F1, EmptyMessageDecoder>,
            // [ErrorKind::Unavailable]
            MessageFieldDecoder<F2, EmptyMessageDecoder>,
            // [ErrorKind::Timeout]
            MessageFieldDecoder<F3, EmptyMessageDecoder>,
            // [ErrorKind::NotLeader]
            MessageFieldDecoder<F4, EmptyMessageDecoder>,
            // [ErrorKind::Unexpected]
            MessageFieldDecoder<F5, ErrorKindUnexpectedDecoder>,
            // [ErrorKind::Other]
            MessageFieldDecoder<F6, EmptyMessageDecoder>,
        )>,
    >,
}
impl_message_decode!(ErrorKindDecoder, ErrorKind, |t: _| {
    Ok(match t {
        Branch6::A(_) => ErrorKind::InvalidInput,
        Branch6::B(_) => ErrorKind::Unavailable,
        Branch6::C(_) => ErrorKind::Timeout,
        Branch6::D(_) => ErrorKind::NotLeader,
        Branch6::E(version) => ErrorKind::Unexpected(version),
        Branch6::F(_) => ErrorKind::Other,
    })
});

/// Encoder for [ErrorKind](../../enum.ErrorKind.html).
#[derive(Debug, Default)]
pub struct ErrorKindEncoder {
    inner: MessageEncoder<
        Oneof<(
            // InvalidInput
            MessageFieldEncoder<F1, EmptyMessageEncoder>,
            // Unavailable
            MessageFieldEncoder<F2, EmptyMessageEncoder>,
            // Timeout
            MessageFieldEncoder<F3, EmptyMessageEncoder>,
            // NotLeader
            MessageFieldEncoder<F4, EmptyMessageEncoder>,
            // Unexpected
            MessageFieldEncoder<F5, ErrorKindUnexpectedEncoder>,
            // Other
            MessageFieldEncoder<F6, EmptyMessageEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(ErrorKindEncoder, ErrorKind, |item: Self::Item| match item {
    ErrorKind::InvalidInput => Branch6::A(()),
    ErrorKind::Unavailable => Branch6::B(()),
    ErrorKind::Timeout => Branch6::C(()),
    ErrorKind::NotLeader => Branch6::D(()),
    ErrorKind::Unexpected(version) => Branch6::E(version),
    ErrorKind::Other => Branch6::F(()),
});

/// Decoder for [ErrorKind::Unexpected](../../enum.ErrorKind.html#variant.Unexpected).
#[derive(Debug, Default)]
pub struct ErrorKindUnexpectedDecoder {
    inner: MessageDecoder<Optional<FieldDecoder<F1, ObjectVersionDecoder>>>,
}
impl_message_decode!(
    ErrorKindUnexpectedDecoder,
    Option<ObjectVersion>,
    |t: Option<u64>| Ok(t.map(ObjectVersion))
);

/// Encoder for [ErrorKind::Unexpected](../../enum.ErrorKind.html#variant.Unexpected).
#[derive(Debug, Default)]
pub struct ErrorKindUnexpectedEncoder {
    inner: MessageEncoder<Optional<FieldEncoder<F1, ObjectVersionEncoder>>>,
}
impl_sized_message_encode!(
    ErrorKindUnexpectedEncoder,
    Option<ObjectVersion>,
    |item: Self::Item| item.map(|v| v.0)
);
