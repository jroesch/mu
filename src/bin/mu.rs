extern crate mu;
extern crate rustc_serialize;
extern crate docopt;
#[macro_use] extern crate log;
extern crate env_logger;

use docopt::Docopt;

const USAGE: &'static str = "
A build tool & package manager for Lean.

Usage:
  mu build
  mu (-h | --help)
  mu --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
}

fn main() {
    // Initializing logging
    env_logger::init().unwrap();

    debug!("main: parsing arguments");

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    debug!("main: parsed arguments={:?}", args);

    let command = match args {
        _ => mu::Command::Build,
    };

    debug!("main: interpreted arguments as command={:?}", command);

    mu::initialize(command).unwrap();
}
