//! AxoneDB storage crate providing persistence via WAL and snapshots.

mod snapshot;
mod wal;

use std::fs;
use std::path::{Path, PathBuf};

use axonedb_core::KvStore;

pub use snapshot::{create as snapshot_create, load as snapshot_load};
pub use wal::Wal;

/// High-level storage facade combining WAL and snapshots.
pub struct Storage {
    wal: Wal,
    snapshot_path: PathBuf,
}

impl Storage {
    /// Initialize storage under `dir`, creating files as needed.
    pub fn new(dir: impl AsRef<Path>) -> std::io::Result<Self> {
        fs::create_dir_all(&dir)?;
        let wal_path = dir.as_ref().join("wal.log");
        let snapshot_path = dir.as_ref().join("snapshot.bin");
        let wal = Wal::open(&wal_path)?;
        Ok(Self { wal, snapshot_path })
    }

    /// Append a log entry to the WAL.
    pub fn log_append(&mut self, entry: &[u8]) -> std::io::Result<()> {
        self.wal.append(entry)
    }

    /// Create a snapshot of the provided `KvStore`.
    pub fn snapshot_create(&self, store: &KvStore) -> std::io::Result<()> {
        snapshot::create(&self.snapshot_path, store)
    }

    /// Load a snapshot into a new `KvStore` instance.
    pub fn snapshot_load(&self) -> std::io::Result<KvStore> {
        snapshot::load(&self.snapshot_path)
    }
}
