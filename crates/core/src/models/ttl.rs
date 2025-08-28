use std::time::{Duration, Instant};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

impl Serialize for Ttl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let remaining = self.deadline.saturating_duration_since(Instant::now());
        serializer.serialize_u64(remaining.as_millis() as u64)
    }
}

impl<'de> Deserialize<'de> for Ttl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Ttl::new(Duration::from_millis(millis)))
    }
}
