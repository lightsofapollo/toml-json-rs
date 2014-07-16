extern crate getopts;
extern crate tomljson;

use std::{os};
use std::io::{File};
use tomljson::JsonConverter;

static USAGE: &'static str = "Convert json file to toml and print to stdout";

fn show_help(opts: &[getopts::OptGroup]) {
  println!("{}", getopts::usage(USAGE, opts));
}

fn main() {
  // main options for the cli
  let opts = [
    getopts::optflag("h", "help", "json-toml"),
  ];

  // validate the arguments
  let args = os::args();

  // default action is to show help
  if args.len() < 2 {
    return show_help(opts);
  }

  let matches = match getopts::getopts(args.tail(), opts) {
    Ok(matches) => { matches },
    Err(err) => fail!(err.to_string())
  };

  // if help was requested show usage and exit.
  if matches.opt_present("h") {
    os::set_exit_status(0);
    show_help(opts);
    return;
  }

  // validate the existance of the given file...
  let toml_path = Path::new(matches.free.get(0).clone());
  if !toml_path.exists() {
    show_help(opts);
    os::set_exit_status(1);
    return;
  }

  // read from the toml file and convert to json
  let mut reader = File::open(&toml_path).unwrap();
  let converter = JsonConverter::new();
  let toml = match converter.convert(&mut reader) {
    Ok(v) => v,
    Err(e) => fail!("Failed to convert json to toml: {}", e)
  };

  println!("{}", toml);
}
