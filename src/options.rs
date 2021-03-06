//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use openalias::Options;
//! let options = Options::parse();
//! println!("Looking up {:?}", options.aliases);
//! ```


use self::super::alias_to_fqdn;
use clap::{AppSettings, Arg};


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Aliases to look up.
    pub aliases: Vec<String>,
    /// Print more information.
    ///
    /// Default: `false`.
    pub verbose: bool,
    /// Just print the record text.
    ///
    /// Default: `false`.
    pub raw: bool,
    /// Limit results to currencies from this list.
    ///
    /// Default: `None`.
    pub currency_filter: Option<Vec<String>>,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<OPEN_ALIAS>... 'Aliases to look up'").validator(Options::open_alias_validator).required(true))
            .arg(Arg::from_usage("-v --verbose 'Print out more information'"))
            .arg(Arg::from_usage("-r --raw 'Print just the record text'"))
            .arg(Arg::from_usage("-c --currency=[CURRENCY]... 'Limit results to just CURRENCY'"))
            .get_matches();

        Options {
            aliases: matches.values_of("OPEN_ALIAS").unwrap().map(String::from).collect(),
            verbose: matches.is_present("verbose"),
            raw: matches.is_present("raw"),
            currency_filter: matches.values_of("currency").map(|cs| cs.map(String::from).collect()),
        }
    }

    fn open_alias_validator(s: String) -> Result<(), String> {
        alias_to_fqdn(&s).map(|_| ()).ok_or_else(|| format!("{} is not a valid OpenAlias address", s))
    }
}
