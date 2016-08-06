extern crate getopts;
extern crate tomljson;

use std::{env, process, path};
use std::fs::File;
use tomljson::JsonConverter;
use getopts::Options;

static USAGE: &'static str = "Convert json file to toml and print to stdout";

fn show_help(opts: Options) {
    println!("{}", opts.usage(USAGE));
}

fn main() {
    // main options for the cli
    let mut opts = Options::new();
    opts.optflag("h", "help", "json-toml");

    // validate the arguments
    let args: Vec<String> = env::args().collect();

    // default action is to show help
    if args.len() < 2 {
        show_help(opts);
        return;
    }

    let matches = match opts.parse(&args) {
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
    let converter = JsonConverter::new();
    let toml = match converter.convert(&mut reader) {
        Ok(v) => v,
        Err(e) => panic!("Failed to convert json to toml: {}", e),
    };

    println!("{}", toml);
}
