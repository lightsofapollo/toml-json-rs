extern crate debug;
extern crate getopts;
extern crate tomljson;

use std::{os};
use std::io::{File, stdio};
use tomljson::TomlConverter;

static USAGE: &'static str = "Convet toml file to json and write it to stdout";

fn show_help(opts: &[getopts::OptGroup]) {
  println!("{}", getopts::usage(USAGE, opts));
}

fn main() {
  // main options for the cli
  let opts = [
    getopts::optflag("h", "help", "Show help for toml-js"),
    getopts::optflag("d", "dense", "Output json as a single line instead a human readable string"),

  ];

  // validate the arguments
  let args = os::args();

  // default action is to show help
  if (args.len() < 2) {
    return show_help(opts);
  }

  let matches = match getopts::getopts(args.tail(), opts) {
    Ok(matches) => { matches },
    Err(err) => fail!(err.to_str())
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
  let converter = TomlConverter::new();
  let json = converter.convert(&mut reader);

  // create a writer to stdout
  let mut stdout = stdio::stdout();

  // pretty print is default since most humans can't read single line json
  if matches.opt_present("d") {
    json.to_writer(&mut stdout);
  } else {
    json.to_pretty_writer(&mut stdout);
  }
}
