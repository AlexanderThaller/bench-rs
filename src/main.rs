extern crate criterion;

use criterion::Criterion;
use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args();
    args.next();

    let command_name = args.next().unwrap();
    let mut command = Command::new(command_name);
    command.args(args);

    Criterion::default().bench_function(format!("{:?}", command).as_str(), |b| {
        b.iter(|| command.output())
    });
}
