#[macro_use]
extern crate log;
extern crate loggerv;

#[macro_use]
extern crate clap;
extern crate criterion;

use clap::App;
use criterion::Criterion;
use std::process::Command;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    {
        let loglevel = value_t!(matches, "log_level", log::LogLevel).unwrap();
        loggerv::init_with_level(loglevel).expect("can not initialize logger with parsed loglevel");
    }
    trace!("matches: {:#?}", matches);

    let (command_name, command_args, sample_size) = {
        let mut command = matches.values_of("command").unwrap();
        (
            command.next().unwrap(),
            command.map(|x| x.into()).collect::<Vec<String>>(),
            value_t!(matches, "sample_size", usize).unwrap(),
        )
    };

    let id = if matches.is_present("id") {
        matches.value_of("id").unwrap().into()
    } else {
        format!("{} {}", command_name, command_args.join(" "))
    };

    Criterion::default()
        .sample_size(sample_size)
        .bench_function(id.as_str(), |b| {
            b.iter(|| create_command(command_name, &command_args).output())
        });
}

fn create_command(name: &str, args: &[String]) -> Command {
    let mut command = Command::new(name);
    command.args(args);
    command
}
