use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use axonedb_core::KvStore;

/// Persist the given `KvStore` to `path` using bincode.
pub fn create(path: impl AsRef<Path>, store: &KvStore) -> io::Result<()> {
    let mut file = File::create(path)?;
    let bytes = bincode::serialize(store).map_err(io::Error::other)?;
    file.write_all(&bytes)
}

/// Load a `KvStore` from the snapshot at `path`.
pub fn load(path: impl AsRef<Path>) -> io::Result<KvStore> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    bincode::deserialize(&buf).map_err(io::Error::other)
}
