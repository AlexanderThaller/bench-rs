# bench-rs
Benchmark command line commands inspired by [bench](https://github.com/Gabriel439/bench).

## Install

Needs nightly right now. To install bench-rs just run the following command:

```
cargo +nightly install bench
```

## CLI Help
```
bench 1.0.0
Alexander Thaller <alexander@thaller.ws>
Run a program, measure execution time and print statistics.

USAGE:
    bench [OPTIONS] <command>...

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -d, --id <id>
            id to save benchmark under. defaults to command value

    -l, --loglevel <level>
            loglevel to run under [default: info]  [values: trace, debug, info, warn,
            error]
    -s, --sample_size <count>
            how many samples to take [default: 100]


ARGS:
    <command>...

```

## Example
