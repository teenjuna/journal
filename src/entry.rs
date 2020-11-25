use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// An entry in a journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub text: String,
    pub date: DateTime<Local>,
}

impl Entry {
    /// Creates new record with given text and date.
    pub fn new(text: &str, date: DateTime<Local>) -> Result<Self, Error> {
        let text = text.trim();
        if text.is_empty() {
            return Err(Error::EmptyText);
        }

        Ok(Self {
            id: date.format("%d.%m.%y").to_string(),
            text: text.to_string(),
            date,
        })
    }
}

/// A storage for entries.
pub trait EntryStorage {
    fn get(&self, id: &str) -> Result<Entry, Error>;
    fn get_all(&self) -> Result<Vec<Entry>, Error>;
    fn save(&mut self, entry: Entry) -> Result<Entry, Error>;
    fn delete(&mut self, id: &str) -> Result<(), Error>;
}

/// All possible entry-related errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("entry text must be not empty")]
    EmptyText,

    #[error("entry {0} is not found")]
    NotFound(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entry() {
        let date = Local::now()
            .timezone()
            .ymd(2020, 12, 31)
            .and_hms(11, 11, 11);

        let text = "     this is some bad \nformatted text    ";
        let entry = Entry::new(text, date).expect("valid args");
        assert_eq!("31.12.20", &entry.id);
        assert_eq!("this is some bad \nformatted text", &entry.text);
        assert_eq!(date, entry.date);

        let text = "     ";
        let err = Entry::new(text, date).expect_err("invalid args");
        assert!(matches!(err, Error::EmptyText));
    }
}
