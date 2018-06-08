extern crate failure;
#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use]
extern crate clap;
extern crate criterion;

use clap::App;
use criterion::Criterion;
use failure::Error;
use std::process::Command;

fn main() {
    if let Err(e) = run() {
        for cause in e.causes() {
            error!("{}", cause);
        }

        trace!("backtrace:\n{}", e.backtrace());

        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    // setup logging
    {
        use simplelog::*;

        TermLogger::init(
            value_t!(matches, "log_level", LogLevelFilter)?,
            Config::default(),
        )?;
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

    Ok(())
}

fn create_command(name: &str, args: &[String]) -> Command {
    let mut command = Command::new(name);
    command.args(args);
    command
}
