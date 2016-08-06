extern crate getopts;
extern crate tomljson;

use getopts::Options;
use std::{env, process, path};
use std::fs::File;
use tomljson::TomlConverter;

static USAGE: &'static str = "Convert toml file to json and write it to stdout";

fn show_help(opts: Options) {
    println!("{}", opts.usage(USAGE));
}

fn main() {
    // main options for the cli
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Show help for toml-js");
    opts.optflag("d",
                 "dense",
                 "Output json as a single line instead a human readable string");

    // validate the arguments
    let args = env::args();

    // default action is to show help
    if args.len() < 2 {
        show_help(opts);
        return;
    }

    let matches = match opts.parse(args) {
        Ok(matches) => matches,
        Err(err) => panic!(err.to_string()),
    };

    // if help was requested show usage and exit.
    if matches.opt_present("h") {
        show_help(opts);
        process::exit(0);
    }

    // validate the existance of the given file...
    let filename = matches.free[0].clone();
    let toml_path = path::Path::new(&filename);
    if !toml_path.exists() {
        show_help(opts);
        process::exit(1);
    }

    // read from the toml file and convert to json
    let mut reader = File::open(&toml_path).unwrap();
    let converter = TomlConverter::new();
    let json = converter.convert(&mut reader);

    // pretty print is default since most humans can't read single line json
    let write = if matches.opt_present("d") {
        json.to_string()
    } else {
        json.pretty().to_string()
    };
    println!("{}", write);
}
