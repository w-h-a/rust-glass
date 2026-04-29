//! Read path: parse query -> plan -> scan -> aggregate -> result.

/// QueryEngine manages the read path:
///   1. Parse query
///   2. Plan: select segments by time range, project columns
///   3. Parallel scan with filter pushdown (rayon)
///   4. Hash aggregation + approximate accumulators
///   5. Merge across segments
///   6. Return result
pub struct QueryEngine {
    // ports injected at construction
}
