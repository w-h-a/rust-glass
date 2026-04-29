//! Port interface for segment persistence.
//! Segments are immutable once written. Read and write, never update.

use std::time::SystemTime;
use crate::domain::segment::Segment;

/// Storage persists and retrieves immutable segments.
pub trait Storage {
    type Error;

    fn write(&self, segment: &Segment) -> Result<(), Self::Error>;
    fn read(&self, id: &str) -> Result<Segment, Self::Error>;
    fn list(&self, start: SystemTime, end: SystemTime) -> Result<Vec<String>, Self::Error>;
    fn delete(&self, id: &str) -> Result<(), Self::Error>;
}
