use anyhow::Result;
use clap::Args;
use journal::entry::EntryStorage;
use journal::Journal;

/// Prints IDs of all saved entries
#[derive(Args)]
pub struct Opts {}

pub fn execute<E: EntryStorage>(
    journal: Journal<E>,
    _opts: Opts,
) -> Result<()> {
    let entries = match &journal {
        Journal::Plain(j) => j.get_entries()?,
    };

    entries.iter().for_each(|e| println!("{}", e.id));

    Ok(())
}
