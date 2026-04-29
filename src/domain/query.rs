use std::collections::HashMap;
use std::time::SystemTime;

/// Query is the observability query model: time range + filters + breakdowns + calculations.
/// Not SQL. Narrow by design.
pub struct Query {
    pub start: SystemTime,
    pub end: SystemTime,
    pub filters: Vec<Filter>,
    pub breakdowns: Vec<String>,
    pub calculations: Vec<Calculation>,
}

/// Filter restricts events by field value.
pub struct Filter {
    pub field: String,
    pub op: FilterOp,
    pub value: String, // parsed at evaluation time
}

pub enum FilterOp {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Exists,
    Contains,
}

/// Calculation is an aggregation to compute over matching events.
pub struct Calculation {
    pub op: CalcOp,
    pub field: String, // empty for Count
}

pub enum CalcOp {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    P50,
    P95,
    P99,
    CountDistinct,
}

/// Result is the output of a query.
pub struct Result {
    pub groups: Vec<Group>,
}

/// Group is one row of query output.
pub struct Group {
    pub keys: HashMap<String, String>,
    pub values: HashMap<String, f64>,
}
