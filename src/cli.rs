use clap::{App, Arg, ArgMatches, AppSettings, SubCommand};

use std::ffi::OsString;
use errors::*;

pub fn parse_command_line<'a, I, T>(arguments: I) -> Result<ArgMatches<'a>, Error>
  where I: 'a + IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
{
  let app = App::new("whim")
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(env!("CARGO_PKG_DESCRIPTION"), " - ", env!("CARGO_PKG_HOMEPAGE")))
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(
      SubCommand::with_name("record")
        .about("connect to GDAX and record real-time market data")
        .arg(Arg::with_name("SANDBOX").long("sandbox"))
    )
    ;

  Ok(app.get_matches_from_safe(arguments)?)
}

#[derive(Debug, PartialEq)]
pub struct Arguments {
  pub command: Command,
}

impl Arguments {
  pub fn from_matches(matches: &ArgMatches) -> Arguments {
    Arguments {
      command: Command::from_matches(matches),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Command {
  Record{sandbox: bool}
}

impl Command {
  pub fn from_matches(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
      ("record", Some(submatches)) => Command::Record{sandbox: submatches.is_present("SANDBOX")},
      (name,     submatches) => panic!("Unexpected subcommand: {} {:?}", name, submatches),
    }
  }
}
