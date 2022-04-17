use crate::entry::{Entry, EntryStorage, Error as EntryError};
use anyhow::{Context, Result};
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};

/// A json-file storage for entries.
pub struct JsonEntryStorage {
    path: PathBuf,
    entries: VecDeque<Entry>,
}

impl JsonEntryStorage {
    pub fn new(path: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let entries = match serde_json::from_reader(&file) {
            Ok(entries) => Ok(entries),
            Err(err) if err.is_eof() => Ok(VecDeque::new()),
            Err(err) => Err(err).context("failed to read json file"),
        }?;

        Ok(Self {
            path: path.to_path_buf(),
            entries,
        })
    }

    fn sync_file(&mut self) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        serde_json::to_writer_pretty(&file, &self.entries)?;

        Ok(())
    }
}

impl EntryStorage for JsonEntryStorage {
    fn get(&self, id: &str) -> Result<Entry, EntryError> {
        match self.entries.iter().find(|e| e.id == id) {
            Some(e) => Ok(e.clone()),
            None => Err(EntryError::NotFound(id.to_string())),
        }
    }

    fn get_all(&self) -> Result<Vec<Entry>, EntryError> {
        Ok(Vec::from(self.entries.to_owned()))
    }

    fn save(&mut self, entry: Entry) -> Result<Entry, EntryError> {
        let entry = match self.entries.iter_mut().find(|e| e.id == entry.id) {
            Some(e) => {
                e.text = entry.text;
                e.date = entry.date;
                e.clone()
            }
            None => {
                self.entries.push_front(entry);
                self.entries[0].clone()
            }
        };

        self.sync_file()?;

        Ok(entry)
    }

    fn delete(&mut self, id: &str) -> Result<(), EntryError> {
        match self.entries.iter().position(|e| e.id == id) {
            Some(i) => {
                self.entries.remove(i);
                self.sync_file()?;

                Ok(())
            }
            None => Err(EntryError::NotFound(id.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Duration;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    fn tmp_path() -> PathBuf {
        let file = NamedTempFile::new().unwrap();
        let path = PathBuf::from(file.path());
        file.close().unwrap();
        path
    }

    #[test]
    fn all() {
        let mut storage = JsonEntryStorage::new(&tmp_path()).unwrap();

        // First, try to get entries.
        let entries = storage.get_all().unwrap();
        assert!(entries.is_empty());

        // And some individual one too.
        let err = storage.get("foo").unwrap_err();
        assert!(matches!(err, EntryError::NotFound(id) if id == "foo"));

        // Save some entries.
        let entry1 = Entry::new("some text", Local::now()).unwrap();
        let entry1 = storage.save(entry1).unwrap();

        let entry2 =
            Entry::new("another text", Local::now() + Duration::days(2))
                .unwrap();
        let entry2 = storage.save(entry2).unwrap();

        // Our list of entries now must contain two.
        let entries = storage.get_all().unwrap();
        assert_eq!(2, entries.len());
        assert_eq!(vec![entry2.clone(), entry1.clone()], entries);

        // Update a record.
        let entry1 = Entry::new("some updated text", entry1.date).unwrap();
        let entry1 = storage.save(entry1).unwrap();

        // And we can easily get some record.
        let entry1_ = storage.get(&entry1.id).unwrap();
        assert_eq!(&entry1, &entry1_);

        // And delete it.
        storage.delete(&entry1.id).unwrap();
        let err = storage.get(&entry1.id).unwrap_err();
        assert!(matches!(err,EntryError::NotFound(id) if id == entry1.id));

        // But delering non-existent entry will return a error.
        let err = storage.delete(&entry1.id).unwrap_err();
        assert!(matches!(err,EntryError::NotFound(id) if id == entry1.id));
    }
}
