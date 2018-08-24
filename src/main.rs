#[macro_use]
extern crate log;
extern crate clap;
extern crate stderrlog;

fn main() {
    use clap::{App, Arg};

    // ==> CMD Argument Parser <==
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Increase message verbosity"))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Silence all output"))
        .arg(Arg::with_name("timestamp")
            .long("timestamp")
            .help("Prepend log lines with a timestamp")
            .takes_value(true)
            .default_value("none")
            .possible_values(&["none", "sec", "ms", "ns"]))
        .arg(Arg::with_name("color")
            .long("color")
            .help("Setup colors output")
            .default_value("auto")
            .takes_value(true)
            .empty_values(false)
            .possible_values(&["always", "always_ansi", "auto", "never"]))
        .get_matches();

    // ==> Setup logger <==
    stderrlog::new()
        .module(module_path!())
        .quiet(matches.is_present("quiet"))
        .verbosity(matches.occurrences_of("verbosity") as usize)
        .timestamp(matches.value_of("timestamp").map(|t|{
            t.parse::<stderrlog::Timestamp>().unwrap_or_else(|_|
                clap::Error {
                    message: format!("invalid value for 'timestamp': '{}'", t).into(),
                    kind: clap::ErrorKind::InvalidValue,
                    info: None
                }.exit()
            )
        }).unwrap_or(stderrlog::Timestamp::Off))
        .color(matches.value_of("color").map(|c|{
            match c.as_ref() {
                "always" => stderrlog::ColorChoice::Always, 
                "always_ansi" => stderrlog::ColorChoice::AlwaysAnsi, 
                "auto" => stderrlog::ColorChoice::Auto, 
                "never" => stderrlog::ColorChoice::Never,
                _ => clap::Error {
                    message: format!("invalid value for 'color': '{}'", c).into(),
                    kind: clap::ErrorKind::InvalidValue,
                    info: None
                }.exit()
            }
        }).unwrap_or(stderrlog::ColorChoice::Never))
        .init()
        .unwrap();
    info!("Logger setup successful");
}