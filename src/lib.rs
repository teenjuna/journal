pub mod entry;
pub mod json;
pub mod plain;

use entry::EntryStorage;
use plain::PlainJournal;

/// Some kind of journal.
pub enum Journal<E> {
    Plain(PlainJournal<E>),
}

impl<E: EntryStorage> Journal<E> {
    pub fn new(entry_storage: E) -> Self {
        // Only plain journal is available right now.
        Self::Plain(PlainJournal::new(entry_storage))
    }
}
