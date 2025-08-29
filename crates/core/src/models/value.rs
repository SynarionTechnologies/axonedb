use std::time::Duration;

use super::ttl::Ttl;
use serde::{Deserialize, Serialize};

/// Stored value along with an optional time-to-live.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Value {
    pub(crate) data: Vec<u8>,
    #[serde(default)]
    ttl: Option<Ttl>,
}

impl Value {
    pub(crate) fn new(data: Vec<u8>, ttl: Option<Duration>) -> Self {
        let ttl = ttl.map(Ttl::new);
        Self { data, ttl }
    }

    pub(crate) fn is_expired(&self) -> bool {
        self.ttl.as_ref().is_some_and(|t| t.is_expired())
    }
}
