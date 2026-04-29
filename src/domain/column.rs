/// ColumnType determines encoding strategy.
pub enum ColumnType {
    /// Dictionary encoding
    String,
    /// Raw encoding
    Int,
    /// Raw encoding
    Float,
    /// RLE encoding
    Bool,
    /// Delta encoding
    Time,
}

/// Column is a typed series of values from a single field across events.
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
    pub encoded: Vec<u8>,
    pub count: u64,
}
