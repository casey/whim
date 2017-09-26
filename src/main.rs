#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate futures;
extern crate chrono;
extern crate extprim;
extern crate clap;
extern crate exit_code;
extern crate num;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;
extern crate websocket;
extern crate regex;

use std::{env, io};
use std::path::PathBuf;
use std::ffi::OsString;
use std::io::prelude::*;

mod errors;
mod cli;
mod record;
mod decimal;
mod gdax;

use errors::*;

use cli::Arguments;
use record::record;

fn run<I, T>(arguments: I, current_dir: Result<PathBuf, io::Error>) -> Result<(), Error>
  where I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone, 
{
  pretty_env_logger::init().chain_err(|| "Failed to initialize env_logger")?;

  let _current_dir = current_dir.chain_err(|| "Bad current directory")?;
  
  let matches = cli::parse_command_line(arguments)?;

  let arguments = Arguments::from_matches(&matches);

  use cli::Command::*;
  match arguments.command {
    Record{sandbox} => record(sandbox),
  }
}

fn main() {
  if let Err(ref e) = run(env::args(), env::current_dir()) {
    let code = e.code();

    if code != exit_code::SUCCESS  {
      let stderr = &mut io::stderr();
      let errmsg = "Error writing to stderr";

      writeln!(stderr, "{}", e).expect(errmsg);

      for e in e.iter().skip(1) {
        writeln!(stderr, "caused by: {}", e).expect(errmsg);
      }

      if let Some(backtrace) = e.backtrace() {
        writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
      }
    }

    std::process::exit(code);
  }
}
