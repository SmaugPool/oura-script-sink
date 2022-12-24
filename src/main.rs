use args::Args;
use clap::Parser;
use env_logger::{Env, DEFAULT_FILTER_ENV};
use oura::Error;

mod args;
mod script_sink;
mod setup;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    env_logger::init_from_env(Env::default().filter_or(
        DEFAULT_FILTER_ENV,
        if args.verbose { "debug" } else { "info" },
    ));

    let threads = setup::oura_bootstrap(args)?;

    for handle in threads {
        handle.join().expect("error in pipeline thread");
    }

    Ok(())
}
