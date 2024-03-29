use oura::{
    model::EventData,
    pipelining::{BootstrapResult, SinkProvider, StageReceiver},
    utils::Utils,
    Error,
};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    utils: Arc<Utils>,
    output: String,
}

impl Config {
    pub fn new(output: String, utils: Arc<Utils>) -> Config {
        Config {
            output: output,
            utils: utils,
        }
    }
}

impl SinkProvider for Config {
    fn bootstrap(&self, input: StageReceiver) -> BootstrapResult {
        let config = self.clone();
        let handle = std::thread::spawn(move || {
            script_writer_loop(input, config).expect("script sink loop failed");
        });

        Ok(handle)
    }
}

pub fn script_writer_loop(input: StageReceiver, config: Config) -> Result<(), Error> {
    for event in input.iter() {
        match event.data {
            EventData::NativeWitness(record) => {
                let json = serde_json::to_string(&record.script_json)?;
                let subdir = &record.policy_id[..2];

                let subdir = Path::new(&config.output).join(subdir);
                let script_path = subdir.join(format!("{}.json", record.policy_id));

                std::fs::create_dir_all(subdir)?;

                if !script_path.exists() {
                    let mut file = File::create(script_path)?;
                    file.write_all(json.as_bytes())?;
                    log::info!("{} {}", record.policy_id, json);
                } else {
                    log::debug!("{} already exists", record.policy_id);
                }
            }
            EventData::BlockEnd(_) => {
                log::trace!(
                    "track_sink_progress(slot={:?} block={:?} hash={:?})",
                    event.context.slot,
                    event.context.block_number,
                    event.context.block_hash
                );
                config.utils.track_sink_progress(&event);
            }
            _ => {}
        }
    }

    Ok(())
}
