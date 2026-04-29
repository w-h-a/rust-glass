use std::time::SystemTime;
use super::column::Column;

/// Segment is a time-partitioned, immutable collection of columns.
/// Once written, a segment is never modified — only deleted when expired.
pub struct Segment {
    pub id: String,
    pub start: SystemTime,
    pub end: SystemTime,
    pub columns: Vec<Column>,
    pub index: SegmentIndex,
}

/// SegmentIndex holds per-column stats for query planning.
pub struct SegmentIndex {
    pub column_stats: Vec<ColumnStat>,
}

/// ColumnStat enables segment skipping during queries.
pub struct ColumnStat {
    pub name: String,
    pub count: u64,
    // TODO: min/max, bloom filter for string columns
}
