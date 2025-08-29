use axonedb_core::KvStore;
use axonedb_storage::{snapshot_create, snapshot_load};
use tempfile::tempdir;

#[test]
fn snapshot_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snapshot.bin");
    let mut store = KvStore::new();
    store.insert(b"k".to_vec(), b"v".to_vec(), None);
    snapshot_create(&path, &store).unwrap();
    let mut loaded = snapshot_load(&path).unwrap();
    assert_eq!(loaded.get(b"k").unwrap(), b"v".to_vec());
}
