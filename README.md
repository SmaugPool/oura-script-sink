# Oura Script Sink

Standalone [Cardano](https://cardano.org) scripts live dumper using [Oura](https://github.com/txpipe/oura) as a library.

For now, [simple scripts](https://github.com/input-output-hk/cardano-node/blob/master/doc/reference/simple-scripts.md), also called native scripts, are supported, [Plutus scripts](https://docs.cardano.org/plutus/Plutus-validator-scripts) are not.

This is useful, among other things, to gather all minting policies.

## Usage

```
Cardano scripts dumper using Oura

USAGE:
    oura-script-sink [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -m, --metrics <METRICS>    Enable Prometheus metrics
                                ('default' for 127.0.0.1:9188/metrics or ADDR:PORT/ENDPOINT)
    -n, --network <NETWORK>    Network ('mainnet' or 'testnet') [default: mainnet]
    -o, --output <OUTPUT>      Output directory [default: /tmp/scripts]
    -s, --socket <SOCKET>      Cardano node socket path [default: ./socket]
    -v, --verbose              Print scripts on standard output
    -V, --version              Print version information
```

For example, assuming a Cardano node running locally with its socket path set to `path/to/socket`:
```
$ oura-script-sink --socket path/to/socket --verbose
```
or with Prometheus metrics and no output (daemon mode):
```
$ oura-script-sink --socket path/to/socket --metrics 0.0.0.0:9188/metrics
```

## Output

Scripts are written to `<OUTPUT>/<PREFIX>/<POLICY_ID.json>`.

`<PREFIX>` subdirectories are the first two characters of the policy IDs and are used to spread the number of files per directory.

## Starting slot & stateful cursor

By default, the process will start crawling from [the first Mary era block](https://cardanoscan.io/block/5406747) then regularly store its "position" using an [Oura cursor](https://txpipe.github.io/oura/advanced/stateful_cursor.html) to avoid reprocessing all blocks on restart.

The cursor file is `<OUTPUT>/cursor` and can be preset to start at a different initial block.
The format is `BLOCK_NUMBER,BLOCK_HASH` without line feed character.

## Snapshot

A snaphot with cursor is available at [pool-pm/cardano-minting-policies](https://github.com/pool-pm/cardano-minting-policies) to sync faster to Cardano tip.

## Metrics
Metrics in [Prometheus](https://prometheus.io/) format can be exposed using the `-m, --metrics` option.

Metrics are disabled by default and `--metrics default` will enable them on `127.0.0.1:9188/metrics`, which will usually make them available only locally. Use `0.0.0.0:9188/metrics` instead to make them available from any network interface.

Example:
```bash
$ curl localhost:9188/metrics

# HELP chain_tip the last detected tip of the chain (height)
# TYPE chain_tip gauge
chain_tip 7106324
# HELP rollback_count number of rollback events occurred
# TYPE rollback_count counter
rollback_count 1
# HELP sink_current_slot last slot processed by the sink of the pipeline
# TYPE sink_current_slot gauge
sink_current_slot 58001490
# HELP sink_event_count number of events processed by the sink of the pipeline
# TYPE sink_event_count counter
sink_event_count 8592
# HELP source_current_height last height (block #) processed by the source of the pipeline
# TYPE source_current_height gauge
source_current_height 7105148
# HELP source_current_slot last slot processed by the source of the pipeline
# TYPE source_current_slot gauge
source_current_slot 58001512
# HELP source_event_count number of events processed by the source of the pipeline
# TYPE source_event_count counter
source_event_count 730951
```

## Build

```bash
$ git clone git@github.com:SmaugPool/oura-script-sink.git

$ cd oura-script-sink
$ cargo install --path .
```

# Support

You can get support in [pool.pm Telegram group](https://t.me/pool_pm).

## License

This project is licensed under the Apache-2.0 license. Please see the [LICENSE](LICENSE.md) file for more details.
