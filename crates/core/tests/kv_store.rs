use axonedb_core::KvStore;
use std::time::Duration;

#[test]
fn insert_get_delete() {
    let mut store = KvStore::new();
    store.insert(b"key".to_vec(), b"val".to_vec(), None);
    assert_eq!(store.get(b"key"), Some(b"val".to_vec()));
    assert!(store.delete(b"key"));
    assert_eq!(store.get(b"key"), None);
}

#[test]
fn ttl_expires() {
    let mut store = KvStore::new();
    store.insert(
        b"key".to_vec(),
        b"val".to_vec(),
        Some(Duration::from_millis(10)),
    );
    assert_eq!(store.get(b"key"), Some(b"val".to_vec()));
    std::thread::sleep(Duration::from_millis(15));
    assert_eq!(store.get(b"key"), None);
}
