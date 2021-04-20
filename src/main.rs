use clap::{
    crate_version,
    load_yaml,
    value_t,
    App,
};
use criterion::Criterion;
use failure::Error;
use log::{
    error,
    trace,
};
use simplelog::{
    ColorChoice,
    Config as LogConfig,
    LevelFilter,
    TermLogger,
    TerminalMode,
};
use std::process::Command;

fn main() {
    if let Err(e) = run() {
        for cause in e.iter_chain() {
            error!("{}", cause);
        }

        trace!("backtrace:\n{}", e.backtrace());

        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    TermLogger::init(
        value_t!(matches, "log_level", LevelFilter)?,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

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
