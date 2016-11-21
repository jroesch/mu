#[macro_use] extern crate log;
extern crate toml;
// extern crate regex;

use std::env;

// mod coqc;
mod error;
// mod package;

// use package::Package;

#[derive(Debug, PartialEq)]
pub enum Command {
    Build,
}

pub fn initialize(command: Command) -> Result<(), error::Error> {
    debug!("initialize: initializing gallus");
    println!("Hello Mu!");
    return Ok(());
    // let current_directory = try!(env::current_dir());
    // let package = try!(package::Package::from_root(&current_directory));
    // debug!("initialize: found package definition {:?}", package);
    // match command {
    //     Command::Build => build(&package),
    // }
}

// pub fn build(package: &package::Package) -> Result<(), error::Error> {
//     let root = &package.root;
//     debug!("build: root={:?} package={:?}", root, package);
//     let coqc = coqc::Coqc { package: package };
//     // let coqdep = compilers::CoqDep { root: root.to_owned() };
//     match coqc.run() {
//         Err(e) => panic!(),
//         Ok(_) => Ok(()),
//     }
// }
