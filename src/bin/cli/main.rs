mod delete;
mod list;
mod read;
mod write;

use anyhow::{anyhow, Context, Result};
use clap::Clap;
use directories::BaseDirs;
use std::fs;
use std::path::Path;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Write(write::Opts),
    List(list::Opts),
    Read(read::Opts),
    Delete(delete::Opts),
}

fn main() -> Result<()> {
    // Setup directory for app data.
    let entries_file = {
        let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("base dirs are not found"))?;
        let data_dir = &base_dirs.data_dir().join("journal");
        fs::create_dir_all(data_dir)?;
        data_dir.join("entries.json")
    };

    // Create journal instance.
    let journal = {
        let entry_storage = journal::json::JsonEntryStorage::new(Path::new(&entries_file))
            .context("failed to open entry storage")?;
        journal::Journal::new(entry_storage)
    };

    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Write(opts) => write::execute(journal, opts)?,
        SubCommand::List(opts) => list::execute(journal, opts)?,
        SubCommand::Read(opts) => read::execute(journal, opts)?,
        SubCommand::Delete(opts) => delete::execute(journal, opts)?,
    }

    Ok(())
}
