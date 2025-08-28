use std::time::{Duration, Instant};

/// Represents a time-to-live deadline for a value.
#[derive(Clone, Debug)]
pub struct Ttl {
    deadline: Instant,
}

impl Ttl {
    /// Create a new [`Ttl`] from the given duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            deadline: Instant::now() + duration,
        }
    }

    /// Returns true if the deadline has passed.
    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.deadline
    }
}
