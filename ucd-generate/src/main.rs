extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate fst;
extern crate ucd_parse;
extern crate ucd_trie;
extern crate ucd_util;

use std::io::{self, Write};
use std::process;

use ucd_parse::{UcdFile, UnicodeData};

use args::ArgMatches;
use error::Result;

macro_rules! eprintln {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
    }}
}

macro_rules! err {
    ($($tt:tt)*) => {
        Err(::error::Error::Other(format!($($tt)*)))
    }
}

mod app;
mod args;
mod error;
mod util;
mod writer;

mod case_folding;
mod general_category;
mod jamo_short_name;
mod names;
mod property_bool;
mod script;

fn main() {
    if let Err(err) = run() {
        if err.is_broken_pipe() {
            process::exit(0);
        }
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = app::app().get_matches();
    match matches.subcommand() {
        ("general-category", Some(m)) => {
            general_category::command(ArgMatches::new(m))
        }
        ("script", Some(m)) => {
            script::command_script(ArgMatches::new(m))
        }
        ("script-extension", Some(m)) => {
            script::command_script_extension(ArgMatches::new(m))
        }
        ("property-bool", Some(m)) => {
            property_bool::command(ArgMatches::new(m))
        }
        ("jamo-short-name", Some(m)) => {
            jamo_short_name::command(ArgMatches::new(m))
        }
        ("names", Some(m)) => {
            names::command(ArgMatches::new(m))
        }
        ("property-names", Some(m)) => {
            cmd_property_names(ArgMatches::new(m))
        }
        ("property-values", Some(m)) => {
            cmd_property_values(ArgMatches::new(m))
        }
        ("case-folding-simple", Some(m)) => {
            case_folding::command(ArgMatches::new(m))
        }
        ("test-unicode-data", Some(m)) => {
            cmd_test_unicode_data(ArgMatches::new(m))
        }
        ("", _) => {
            app::app().print_help()?;
            println!("");
            Ok(())
        }
        (unknown, _) => err!("unrecognized command: {}", unknown),
    }
}

fn cmd_property_names(args: ArgMatches) -> Result<()> {
    use util::PropertyNames;

    let dir = args.ucd_dir()?;
    let names = PropertyNames::from_ucd_dir(&dir)?;

    let mut wtr = args.writer("property_names")?;
    wtr.string_to_string(args.name(), &names.0)?;
    Ok(())
}

fn cmd_property_values(args: ArgMatches) -> Result<()> {
    use util::PropertyValues;

    let dir = args.ucd_dir()?;
    let values = PropertyValues::from_ucd_dir(&dir)?;

    let mut wtr = args.writer("property_values")?;
    wtr.string_to_string_to_string(args.name(), &values.value)?;
    Ok(())
}

fn cmd_test_unicode_data(args: ArgMatches) -> Result<()> {
    let dir = args.ucd_dir()?;
    let mut stdout = io::stdout();
    for result in UnicodeData::from_dir(dir)? {
        let x: UnicodeData = result?;
        writeln!(stdout, "{}", x)?;
    }
    Ok(())
}
