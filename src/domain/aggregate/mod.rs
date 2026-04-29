//! Approximate accumulators.
//! t-digest for percentiles, HyperLogLog for count-distinct.
//! Merge is commutative, associative — same properties as CRDTs.
//! Pure functions: no I/O, no side effects.

/// Accumulator computes an approximate aggregate that can merge across segments.
pub trait Accumulator {
    fn add(&mut self, value: f64);
    fn merge(&mut self, other: &Self);
    fn result(&self) -> f64;
}
