use std::collections::HashMap;
use super::column::ColumnType;
use super::event::Event;

/// Schema is the discovered set of columns with types and cardinality.
/// Grows monotonically — new columns appear, columns never disappear.
/// No DDL. Discovered from data at flush time.
pub struct Schema {
    pub columns: HashMap<String, ColumnMeta>,
}

/// ColumnMeta tracks a discovered column's type and cardinality estimate.
pub struct ColumnMeta {
    pub name: String,
    pub column_type: ColumnType,
    pub cardinality: u64,
}

/// Discover infers schema from a batch of events.
pub fn discover(_events: &[Event]) -> Schema {
    Schema {
        columns: HashMap::new(),
    } // TODO
}
