use args::Args;
use clap::Parser;
use oura::Error;
use std::path::Path;

mod args;
mod script_sink;
mod setup;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if !Path::new(&args.socket).exists() {
        return Err("socket not found".into());
    }

    let threads = setup::oura_bootstrap(args)?;

    for handle in threads {
        handle.join().expect("error in pipeline thread");
    }

    Ok(())
}
