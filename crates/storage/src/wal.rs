use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Simple write-ahead log backed by a file.
pub struct Wal {
    file: File,
    path: PathBuf,
}

impl Wal {
    /// Open or create a WAL at the given `path`.
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&path)?;
        Ok(Self { file, path })
    }

    /// Append a binary entry to the log.
    pub fn append(&mut self, entry: &[u8]) -> io::Result<()> {
        let len = entry.len() as u64;
        self.file.write_all(&len.to_le_bytes())?;
        self.file.write_all(entry)?;
        self.file.sync_data()
    }

    /// Create an iterator over log entries from the start.
    pub fn iter(&self) -> io::Result<WalIterator> {
        let mut file = OpenOptions::new().read(true).open(&self.path)?;
        file.seek(SeekFrom::Start(0))?;
        Ok(WalIterator { file })
    }
}

/// Iterator yielding entries from a WAL.
pub struct WalIterator {
    file: File,
}

impl Iterator for WalIterator {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut len_buf = [0u8; 8];
        match self.file.read_exact(&mut len_buf) {
            Ok(()) => {
                let len = u64::from_le_bytes(len_buf) as usize;
                let mut buf = vec![0; len];
                if let Err(e) = self.file.read_exact(&mut buf) {
                    return Some(Err(e));
                }
                Some(Ok(buf))
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
