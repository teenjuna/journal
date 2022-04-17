use anyhow::Result;
use clap::Args;
use journal::entry::{Entry, EntryStorage};
use journal::Journal;

/// Prints IDs of all saved entries
#[derive(Args)]
pub struct Opts {
    /// List entries in reverse order.
    #[clap(short, long)]
    reverse: bool,
}

pub fn execute<E: EntryStorage>(journal: Journal<E>, opts: Opts) -> Result<()> {
    let entries = match &journal {
        Journal::Plain(j) => j.get_entries()?,
    };

    let entries: Box<dyn Iterator<Item = &Entry>> = if opts.reverse {
        // From newer to older.
        Box::new(entries.iter())
    } else {
        // From older to newer by default.
        Box::new(entries.iter().rev())
    };

    entries.for_each(|e| println!("{}", e.id));

    Ok(())
}
