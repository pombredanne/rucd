// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//  ucd-generate case-folding-simple /home/andrew/tmp/ucd-10.0.0/ --chars --circular --fst-dir ../ucd-util/src/tables/fst
//
// ucd-generate is available on crates.io.

lazy_static! {
  pub static ref CASE_FOLDING_SIMPLE: ::fst::Map = 
    ::fst::Map::from(::fst::raw::Fst::from_static_slice(
      include_bytes!("case_folding_simple.fst")).unwrap());
}