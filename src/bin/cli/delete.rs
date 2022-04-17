use anyhow::Result;
use clap::Args;
use journal::entry::EntryStorage;
use journal::Journal;

/// Deletes an entry
#[derive(Args)]
pub struct Opts {
    /// ID of the entry to delete
    #[clap(name = "ID")]
    id: String,
}

pub fn execute<E: EntryStorage>(
    mut journal: Journal<E>,
    opts: Opts,
) -> Result<()> {
    match &mut journal {
        Journal::Plain(j) => j.delete_entry(&opts.id)?,
    };

    Ok(())
}
