mod write;

use anyhow::Result;
use clap::Clap;
use std::path::Path;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Write(write::Opts),
}

fn main() -> Result<()> {
    // Create journal instance.
    let journal = {
        let entry_storage = journal::json::JsonEntryStorage::new(Path::new("/tmp/entries.json"))?;
        journal::Journal::new(entry_storage)
    };

    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Write(opts) => write::execute(journal, opts)?,
    }

    Ok(())
}
