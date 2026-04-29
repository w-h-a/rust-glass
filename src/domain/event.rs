use std::collections::HashMap;
use std::time::SystemTime;

/// Event is an arbitrarily wide structured event — the only primitive.
/// Every span, log, or metric is an Event with different fields.
pub struct Event {
    pub timestamp: SystemTime,
    pub trace_id: String,
    pub span_id: String,
    pub fields: HashMap<String, Value>,
}

/// Value represents a dynamically typed field value.
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}
