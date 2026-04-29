//! Write path: OTLP -> buffer -> encode -> segment.

/// Ingester manages the write path:
///   1. Receive events via OTLP
///   2. Buffer in memory, batched by time
///   3. Flush: encode columns, write immutable segment
///   4. Update schema registry
pub struct Ingester {
    // ports injected at construction
}
