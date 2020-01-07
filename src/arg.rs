extern crate clap;

use clap::{App, Arg};
use std::fmt;

pub struct Args {
    pub is_dry_run: bool,
    pub keep_count: usize,
    pub keep_count_snapshot: usize
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Args(is_dry_run: {}, keep_count: {}, keep_count_snapshot: {})",
            self.is_dry_run, self.keep_count, self.keep_count_snapshot
        )
    }
}

fn parse_int(s: &str, var_name: &str) -> usize {
    match s.parse() {
        Result::Ok(val) => val,
        Result::Err(err) =>
            panic!("{} should be integer. {:?}", var_name, err),
    }
}
    
pub fn parse_arg() -> Args {
    let app = App::new("trimimages")
        .version("0.1")
        .author("Shisei Hanai<ruimo.uno@gmail.com>")
        .about("Remove docker images")
        .arg(Arg::with_name("dryrun")
             .help("Dry run. Just show docker commands to execute.")
             .long("dry-run")
        )
        .arg(Arg::with_name("keep")
             .help("Keep count for canonical versioned image.")
             .long("keep")
             .default_value("3")
        )
        .arg(Arg::with_name("keep-snapshot")
             .help("Keep count for snapshot versioned image.")
             .long("keep-snapshot")
             .default_value("1")
        );

    let matches = app.get_matches();

    Args {
        is_dry_run: matches.is_present("dryrun"),
        keep_count: parse_int(matches.value_of("keep").unwrap(), "keep count"),
        keep_count_snapshot: parse_int(matches.value_of("keep-snapshot").unwrap(), "keep count snapshot")
    }
}
