use std::path::Path;
use std::{str::FromStr, sync::Arc, thread::JoinHandle};

use oura::{
    filters::selection::{self, Predicate},
    pipelining::{
        BootstrapResult, FilterProvider, PartialBootstrapResult, SinkProvider, SourceProvider,
        StageReceiver,
    },
    sources::{n2c, n2n, AddressArg, BearerKind, IntersectArg, MagicArg, PointArg},
    utils::{cursor, ChainWellKnownInfo, Utils, WithUtils},
};

use crate::args::{Args, Metrics};
use crate::script_sink::Config as ScriptSinkConfig;

const MARY_FIRST_BLOCK_NO: u64 = 23068800;
static MARY_FIRST_BLOCK_HASH: &str =
    "a650a3f398ba4a9427ec8c293e9f7156d81fd2f7ca849014d8d2c1156c359b3a";

pub type Error = Box<dyn std::error::Error>;

pub fn oura_bootstrap(args: Args) -> Result<Vec<JoinHandle<()>>, Error> {
    let magic = MagicArg::from_str(&args.network)?;
    let utils = get_utils(*magic, &args.output, args.metrics)?;

    let (source_handle, source_rx) = match (args.socket, args.host) {
        (Some(socket), None) => {
            bootstrap_n2c_source(AddressArg(BearerKind::Unix, socket), magic, utils.clone())?
        }
        (None, Some(host)) => {
            let addr = format!("{}:{}", host, args.port);
            bootstrap_n2n_source(AddressArg(BearerKind::Tcp, addr), magic, utils.clone())?
        }
        _ => return Err("invalid node options".into()),
    };

    let (filter_handle, filter_rx) = bootstrap_filter(source_rx)?;
    let sink_handle = bootstrap_sink(args.output, utils, filter_rx, args.verbose)?;

    Ok(vec![source_handle, filter_handle, sink_handle])
}

fn get_utils(magic: u64, output: &String, metrics: Option<Metrics>) -> Result<Arc<Utils>, Error> {
    let well_known = ChainWellKnownInfo::try_from_magic(magic)?;

    let cursor_path = Path::new(&output).join(String::from("cursor"));
    let cursor_path_str = cursor_path.to_string_lossy();
    let cursor_config = cursor::Config::File(cursor::FileConfig {
        path: cursor_path_str.into_owned(),
    });

    let utils = Utils::new(well_known).with_cursor(cursor_config);

    Ok(Arc::new(match metrics {
        Some(metrics) => utils.with_metrics((*metrics).clone()),
        None => utils,
    }))
}

fn bootstrap_n2c_source(
    address: AddressArg,
    magic: MagicArg,
    utils: Arc<Utils>,
) -> PartialBootstrapResult {
    #[allow(deprecated)]
    let source_config = n2c::Config {
        address: address,
        magic: Some(magic),
        well_known: None,
        mapper: Default::default(),
        since: None,
        min_depth: 0,
        intersect: Some(IntersectArg::Point(PointArg(
            MARY_FIRST_BLOCK_NO,
            MARY_FIRST_BLOCK_HASH.to_owned(),
        ))),
        retry_policy: None,
    };

    let source_setup = WithUtils::new(source_config, utils);
    let (source_handle, source_rx) = source_setup.bootstrap()?;

    Ok((source_handle, source_rx))
}

fn bootstrap_n2n_source(
    address: AddressArg,
    magic: MagicArg,
    utils: Arc<Utils>,
) -> PartialBootstrapResult {
    #[allow(deprecated)]
    let source_config = n2n::Config {
        address: address,
        magic: Some(magic),
        well_known: None,
        mapper: Default::default(),
        since: None,
        min_depth: 0,
        intersect: Some(IntersectArg::Point(PointArg(
            MARY_FIRST_BLOCK_NO,
            MARY_FIRST_BLOCK_HASH.to_owned(),
        ))),
        retry_policy: None,
        finalize: None,
    };

    let source_setup = WithUtils::new(source_config, utils);
    let (source_handle, source_rx) = source_setup.bootstrap()?;

    Ok((source_handle, source_rx))
}

fn bootstrap_filter(source_rx: StageReceiver) -> PartialBootstrapResult {
    let check = Predicate::VariantIn(vec!["NativeWitness".to_owned()]);
    let filter_setup = selection::Config { check };
    let (filter_handle, filter_rx) = filter_setup.bootstrap(source_rx)?;

    Ok((filter_handle, filter_rx))
}

fn bootstrap_sink(
    output: String,
    utils: Arc<Utils>,
    filter_rx: StageReceiver,
    verbose: bool,
) -> BootstrapResult {
    let sink_setup = ScriptSinkConfig::new(output, utils, verbose);
    let sink_handle = sink_setup.bootstrap(filter_rx)?;

    Ok(sink_handle)
}
