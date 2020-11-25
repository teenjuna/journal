use crate::entry::{Entry, EntryStorage, Error as EntryError};

/// A simple, non-encrypted journal.
pub struct PlainJournal<E> {
    entry_storage: E,
}

impl<E: EntryStorage> PlainJournal<E> {
    /// Returns new plain journal.
    pub fn new(entry_storage: E) -> Self {
        Self { entry_storage }
    }

    /// Returns entry by given `id`.
    pub fn get_entry(&self, id: &str) -> Result<Entry, Error> {
        self.entry_storage.get(id).map_err(|e| e.into())
    }

    /// Returns all entries.
    pub fn get_entries(&self) -> Result<Vec<Entry>, Error> {
        self.entry_storage.get_all().map_err(|e| e.into())
    }

    /// Saves given `entry`.
    pub fn save_entry(&mut self, entry: Entry) -> Result<Entry, Error> {
        self.entry_storage.save(entry).map_err(|e| e.into())
    }

    /// Deletes entry by given `id`.
    pub fn delete_entry(&mut self, id: &str) -> Result<(), Error> {
        self.entry_storage.delete(id).map_err(|e| e.into())
    }
}

/// All possible plain-journal-related errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    EntryError(#[from] EntryError),
}
