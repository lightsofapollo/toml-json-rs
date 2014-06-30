extern crate debug;
extern crate getopts;
extern crate tomljson;

use std::{os};
use std::io::{File, stdio};
use tomljson::TomlConverter;

static USAGE: &'static str = "Convert a toml file to 2 space indented json file.";

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

  if matches.opt_present("d") {
    // because most humans like pretty printed
    json.to_writer(&mut stdout);
  } else {
    json.to_pretty_writer(&mut stdout);
  }
}
