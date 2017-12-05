extern crate criterion;

use criterion::Criterion;
use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args();
    args.next();

    let command_name = args.next().unwrap();
    let command_args: Vec<_> = args.map(|x| x.into()).collect();

    Criterion::default().sample_size(10).bench_function(
        format!("{} {}", command_name, command_args.join(" ")).as_str(),
        |b| {
            b.iter(|| {
                create_command(command_name.as_str(), &command_args).output()
            })
        },
    );
}

fn create_command(name: &str, args: &[String]) -> Command {
    let mut command = Command::new(name);
    command.args(args);
    command
}
