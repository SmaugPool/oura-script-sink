use args::Args;
use clap::Parser;
use oura::Error;

mod args;
mod script_sink;
mod setup;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let threads = setup::oura_bootstrap(args)?;

    for handle in threads {
        handle.join().expect("error in pipeline thread");
    }

    Ok(())
}
