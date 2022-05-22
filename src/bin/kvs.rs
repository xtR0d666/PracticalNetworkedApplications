use clap::{Arg, Command};
use std::process::exit;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("set")
                .arg(Arg::new("key"))
                .arg(Arg::new("value"))
                .about("set the value of a given key"),
        )
        .subcommand(
            Command::new("get")
                .arg(Arg::new("key"))
                .about("get the string of a given key"),
        )
        .subcommand(
            Command::new("rm")
                .arg(Arg::new("key"))
                .about("remove the key-value pair"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", _)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("get", _)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("rm", _)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
