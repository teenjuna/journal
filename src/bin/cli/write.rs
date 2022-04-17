use anyhow::{anyhow, Result};
use chrono::Local;
use clap::Args;
use journal::entry::{Entry, EntryStorage};
use journal::Journal;
use std::process::Command;
use std::{env, fs};
use tempfile::NamedTempFile;

/// Opens an editor for the today's entry
#[derive(Args)]
pub struct Opts {}

pub fn execute<E: EntryStorage>(
    mut journal: Journal<E>,
    _opts: Opts,
) -> Result<()> {
    // Get the previous version of the today's entry (if exists).
    let id = Local::now().format("%d.%m.%y").to_string();
    let prev_text = match &journal {
        Journal::Plain(j) => match j.get_entry(&id) {
            Ok(e) => e.text,
            Err(jerr) => match jerr {
                journal::plain::Error::EntryError(ref eerr) => match eerr {
                    journal::entry::Error::NotFound(_) => "".to_string(),
                    _ => return Err(anyhow!(jerr)),
                },
            },
        },
    };

    // Create a temporary file for entry content with previous text.
    let file = NamedTempFile::new()?;
    fs::write(file.path(), prev_text)?;

    // Open the file in the $EDITOR.
    let editor = get_editor_command()?;
    Command::new(editor).arg(file.path()).spawn()?.wait()?;

    // Read the content of the file.
    let content = fs::read_to_string(file.path())?;
    let text = content.trim();

    // Create an entry.
    let date = Local::now();
    let entry = Entry::new(text, date)?;

    // Save the entry.
    match &mut journal {
        Journal::Plain(j) => j.save_entry(entry)?,
    };

    Ok(())
}

fn get_editor_command() -> Result<String> {
    match env::var("EDITOR") {
        Ok(editor) => {
            if editor.trim() == "" {
                Err(anyhow!("EDITOR variable is empty"))
            } else {
                Ok(editor)
            }
        }
        Err(err) => match err {
            env::VarError::NotPresent => {
                Err(anyhow!("EDITOR variable is not present"))
            }
            env::VarError::NotUnicode(_) => {
                Err(anyhow!("EDITOR variable contains invalid data"))
            }
        },
    }
}
