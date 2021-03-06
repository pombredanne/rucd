/*!
A library for parsing the Unicode character database.
*/

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub use common::{
    UcdFile, Codepoint, Codepoints, UcdLineParser, CodepointIter,
    parse, parse_by_codepoint, parse_many_by_codepoint,
};
pub use error::{Error, ErrorKind};

pub use age::Age;
pub use case_folding::{CaseFold, CaseStatus};
pub use core_properties::CoreProperty;
pub use jamo_short_name::JamoShortName;
pub use name_aliases::{NameAlias, NameAliasLabel};
pub use prop_list::Property;
pub use property_aliases::PropertyAlias;
pub use property_value_aliases::PropertyValueAlias;
pub use script_extensions::ScriptExtension;
pub use scripts::Script;
pub use special_casing::SpecialCaseMapping;
pub use unicode_data::{
    UnicodeData, UnicodeDataNumeric,
    UnicodeDataDecomposition, UnicodeDataDecompositionTag,
    UnicodeDataExpander,
};

macro_rules! err {
    ($($tt:tt)*) => {
        Err(::error::error_parse(format!($($tt)*)))
    }
}

mod common;
mod error;

mod age;
mod case_folding;
mod core_properties;
mod jamo_short_name;
mod name_aliases;
mod prop_list;
mod property_aliases;
mod property_value_aliases;
mod script_extensions;
mod scripts;
mod special_casing;
mod unicode_data;
