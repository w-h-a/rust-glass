//! Column encoders and decoders.
//! Pure transforms: typed values -> bytes (encode), bytes -> typed values (decode).
//! Dictionary for strings, delta for timestamps, RLE for booleans, raw for numbers.

use super::event::Value;

/// Encoder encodes a slice of typed values into bytes.
pub trait Encoder {
    fn encode(&self, values: &[Value]) -> Result<Vec<u8>, EncodeError>;
}

/// Decoder decodes bytes back into typed values.
pub trait Decoder {
    fn decode(&self, data: &[u8]) -> Result<Vec<Value>, DecodeError>;
}

#[derive(Debug)]
pub struct EncodeError;

#[derive(Debug)]
pub struct DecodeError;
