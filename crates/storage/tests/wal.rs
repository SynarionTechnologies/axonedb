use axonedb_storage::Wal;
use tempfile::tempdir;

#[test]
fn replay_returns_appended_entries() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("wal.log");
    let mut wal = Wal::open(&path).unwrap();
    wal.append(b"one").unwrap();
    wal.append(b"two").unwrap();
    let entries: Vec<_> = wal.iter().unwrap().map(|e| e.unwrap()).collect();
    assert_eq!(entries, vec![b"one".to_vec(), b"two".to_vec()]);
}
