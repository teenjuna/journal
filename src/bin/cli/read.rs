use anyhow::Result;
use clap::Args;
use journal::entry::EntryStorage;
use journal::Journal;

/// Opens an entry
#[derive(Args)]
pub struct Opts {
    /// ID of the entry to open
    #[clap(name = "ID")]
    id: String,
}

pub fn execute<E: EntryStorage>(journal: Journal<E>, opts: Opts) -> Result<()> {
    let entry = match &journal {
        Journal::Plain(j) => j.get_entry(&opts.id)?,
    };

    println!("{}", entry.text);

    Ok(())
}
