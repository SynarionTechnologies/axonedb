use axonedb_core::KvStore;
use axonedb_storage::{Storage, Wal};
use tempfile::tempdir;

#[test]
fn storage_persists_log_and_snapshot() {
    let dir = tempdir().unwrap();
    let mut storage = Storage::new(dir.path()).unwrap();
    storage.log_append(b"entry").unwrap();

    let entries: Vec<_> = Wal::open(dir.path().join("wal.log"))
        .unwrap()
        .iter()
        .unwrap()
        .map(|e| e.unwrap())
        .collect();
    assert_eq!(entries, vec![b"entry".to_vec()]);

    let mut store = KvStore::new();
    store.insert(b"k".to_vec(), b"v".to_vec(), None);
    storage.snapshot_create(&store).unwrap();
    let mut loaded = storage.snapshot_load().unwrap();
    assert_eq!(loaded.get(b"k").unwrap(), b"v".to_vec());
}
