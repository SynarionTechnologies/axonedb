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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn snapshot_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("snapshot.bin");
        let mut store = KvStore::new();
        store.insert(b"k".to_vec(), b"v".to_vec(), None);
        create(&path, &store).unwrap();
        let mut loaded = load(&path).unwrap();
        assert_eq!(loaded.get(b"k").unwrap(), b"v".to_vec());
    }
}
